//! GGA_C_GAPC lxc pol — lxc_pol part 33 (v4rho2sigma2_12) CSE chunk 946/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part33_v4rho2sigma2_12_chunk946(t11534: f64, t11558: f64, t1026: f64, t632: f64, t3018: f64, t3022: f64, t3691: f64, t3679: f64, t5248: f64, t1643: f64, t3683: f64, t424: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11559 = t11534 * t11558;
    let t11561 = t632 * t1026;
    let t11562 = t11561 * t3018;
    let t11564 = t3691 * t3022;
    let t11566 = t3679 * t5248;
    let t11567 = t1643 * t11566;
    let t11569 = t424 * t3683;
    (t11559, t11561, t11562, t11564, t11566, t11567, t11569)
}
