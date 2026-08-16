//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 804/1331 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk804(t122: f64, t2674: f64, t2995: f64, t134: f64, t2254: f64, t941: f64, t3405: f64, t3297: f64, t9552: f64, t2580: f64, t9166: f64, t2578: f64) -> (f64, f64, f64, f64, f64) {
    let t9574 = t2674 * t122;
    let t9575 = t9574 * t2995;
    let t9576 = t134 * t2254;
    let t9577 = t941 * t9576;
    let t9578 = t3405 * t9577;
    let t9579 = t9575 * t9578;
    let t9581 = t9552 * t3297;
    let t9583 = t9166 * t2580;
    let t9584 = t2578 * t9583;
    (t9574, t9578, t9579, t9581, t9584)
}
