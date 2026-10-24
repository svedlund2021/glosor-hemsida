use
 axum::Router;
use
 tower_http::services::ServeDir;
use
 std::net::SocketAddr;

#[tokio::main]
async fn
 main() {
    // Serve your frontend HTML assets directly from the static folder


    let app = Router::new().fallback_service(ServeDir::new("Static"
));


    // Render automatically assigns port 10000 to web instances


    let addr = SocketAddr::from(([0, 0, 0, 0], 10000
));
    println!("Server starting up on {}"
, addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await
.unwrap();
    axum::serve(listener, app).await
.unwrap();
}