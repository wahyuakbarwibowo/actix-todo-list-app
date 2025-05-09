mod entity;

use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use dotenvy::dotenv;
use entity::todos::{self, Entity as Todos};
use sea_orm::{ActiveModelTrait, Database, EntityTrait, Set};
use std::env;

#[derive(serde::Deserialize)]
struct CreateTodo {
    title: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let db = Database::connect(&database_url)
        .await
        .expect("Failed to connect to DB");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone()))
            .route("/todos", web::get().to(get_todos))
            .route("/todos", web::post().to(create_todo))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

async fn get_todos(db: web::Data<sea_orm::DatabaseConnection>) -> impl Responder {
    let todos = Todos::find()
        .all(db.get_ref())
        .await
        .expect("Error fetching todos");

    HttpResponse::Ok().json(todos)
}

async fn create_todo(
    db: web::Data<sea_orm::DatabaseConnection>,
    todo: web::Json<CreateTodo>,
) -> impl Responder {
    let new_todo = todos::ActiveModel {
        title: Set(todo.title.clone()),
        ..Default::default()
    };

    let res = new_todo.insert(db.get_ref()).await;

    match res {
        Ok(model) => HttpResponse::Ok().json(model),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
