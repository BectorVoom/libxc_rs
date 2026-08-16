//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 955/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk955(t3696: f64, t3703: f64, t424: f64, t134: f64, t3698: f64, t3702: f64, t11534: f64, t1026: f64, t632: f64, t3018: f64, t3022: f64, t3691: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11555 = t424 * t3696 * t3703;
    let t11557 = t3698 * t134;
    let t11558 = t11557 * t3702;
    let t11559 = t11534 * t11558;
    let t11561 = t632 * t1026;
    let t11562 = t11561 * t3018;
    let t11564 = t3691 * t3022;
    (t11555, t11557, t11558, t11559, t11561, t11562, t11564)
}
