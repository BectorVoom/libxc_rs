//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3192/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3192(t1261: f64, t1715: f64, t247: f64, t44701: f64, t1214: f64, t17748: f64, t17754: f64, t12809: f64, t12916: f64, t17380: f64, t3568: f64, t5333: f64) -> (f64, f64, f64, f64, f64) {
    let t58777 = t1261 * t247 * t44701 * t1715;
    let t58780 = t17748 * t1214;
    let t58785 = t17754 * t1214;
    let t58791 = t12809 * t12916 * t17380;
    let t58793 = t5333 * t3568;
    (t58777, t58780, t58785, t58791, t58793)
}
