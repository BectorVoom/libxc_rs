//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 819/1308 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk819(t122: f64, t2786: f64, t2995: f64, t3408: f64, t3363: f64, t3415: f64, t1081: f64, t2648: f64, t2594: f64, t9408: f64, t2674: f64, t134: f64, t2254: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9563 = t2786 * t122;
    let t9564 = t9563 * t2995;
    let t9565 = t9564 * t3408;
    let t9567 = t3363 * t2995;
    let t9568 = t9567 * t3415;
    let t9570 = t1081 * t2648;
    let t9572 = t9408 * t2594;
    let t9574 = t2674 * t122;
    let t9575 = t9574 * t2995;
    let t9576 = t134 * t2254;
    (t9563, t9565, t9568, t9570, t9572, t9574, t9575, t9576)
}
