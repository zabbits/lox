#![allow(dead_code)]
use crate::token::{LiteralValue, Token};

pub trait Visitor<R> {
    fn visit(&self, expr: &Expr) -> R;
}
pub trait ExprVisitor<R> {
    fn visit_literal(&self, expr: &LiteralExpr) -> R;
    fn visit_unary(&self, expr: &UnaryExpr) -> R;
    fn visit_binary(&self, expr: &BinaryExpr) -> R;
    fn visit_grouping(&self, expr: &GroupingExpr) -> R;
}
impl<T, R> Visitor<R> for T
where
    T: ExprVisitor<R>,
{
    fn visit(&self, expr: &Expr) -> R {
        match expr {
            Expr::Literal(v) => self.visit_literal(v),
            Expr::Unary(v) => self.visit_unary(v),
            Expr::Binary(v) => self.visit_binary(v),
            Expr::Grouping(v) => self.visit_grouping(v),
        }
    }
}
#[derive(Debug)]
pub enum Expr {
    Literal(Box<LiteralExpr>),
    Unary(Box<UnaryExpr>),
    Binary(Box<BinaryExpr>),
    Grouping(Box<GroupingExpr>),
}
impl Expr {
    pub fn visit<R>(&self, visitor: &dyn Visitor<R>) -> R {
        visitor.visit(self)
    }
}
#[derive(Debug)]
pub struct LiteralExpr {
    pub value: LiteralValue,
}
#[derive(Debug)]
pub struct UnaryExpr {
    pub operator: Token,
    pub right: Expr,
}
#[derive(Debug)]
pub struct BinaryExpr {
    pub left: Expr,
    pub right: Expr,
    pub operator: Token,
}
#[derive(Debug)]
pub struct GroupingExpr {
    pub expression: Expr,
}
