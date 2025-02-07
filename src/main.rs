use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};



async fn greet(req:HttpRequest)-> impl Responder{
    let name= req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!",&name)

}

async fn health_check(req:HttpRequest)->impl Responder{
    HttpResponse::Ok()
}



#[tokio::main]

async fn main() ->Result<(),std::io::Error>{
    HttpServer::new(||{
        App::new().route("/",web::get().to(greet))
        .route("/health_check", web::get().to(health_check))
        .route("/{name}", web::get().to(greet))
    }).bind("127.0.0.1:8000")?
    .run()
    .await
}

// #[cfg(test)]
// mod tests{
//     use crate::health_check;
//     #[tokio::test]
//     async fn health_check_succeeds(){
//         let response=health_check().await;
//         assert!(response.status().is_success())
//     }
// }