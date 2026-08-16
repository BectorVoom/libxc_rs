//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 813/1427 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk813(t5462: f64, t612: f64, t1671: f64, t5549: f64, t3638: f64, t458: f64, t1653: f64, t122: f64, t1803: f64, t2995: f64, t1303: f64, t134: f64) -> (f64, f64, f64, f64, f64) {
    let t9297 = t5462 * t612;
    let t9298 = t1671 * t5549;
    let t9299 = t9297 * t9298;
    let t9301 = t3638 * t458;
    let t9302 = t1653 * t9301;
    let t9304 = t1803 * t122;
    let t9305 = t9304 * t2995;
    let t9306 = t134 * t1303;
    (t9299, t9302, t9304, t9305, t9306)
}
