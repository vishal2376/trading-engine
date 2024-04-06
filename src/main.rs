#[derive(Debug)]
enum BidOrAsk {
    Bid,
    Ask,
}

#[derive(Debug)]
struct Limit {
    price: f64,
    orders: Vec<Order>,
}

impl Limit {
    fn new(price: f64) -> Limit {
        Limit {
            price,
            orders: Vec::new(),
        }
    }

    fn add_order(&mut self, order: Order) {
        self.orders.push(order);
    }
}

#[derive(Debug)]
struct Order {
    size: f64,
    bid_or_ask: BidOrAsk,
}

impl Order {
    fn new(bid_or_ask: BidOrAsk, size: f64) -> Order {
        Order { size, bid_or_ask }
    }
}

fn main() {
    let mut limit = Limit::new(100.0);
    let first_order = Order::new(BidOrAsk::Bid, 10.0);

    limit.add_order(first_order);

    println!("{:?}", limit);
}
