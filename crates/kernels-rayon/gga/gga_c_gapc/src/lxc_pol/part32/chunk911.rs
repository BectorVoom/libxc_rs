//! GGA_C_GAPC lxc pol — lxc_pol part 32 (v4rho2sigma2_11) CSE chunk 911/1311 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part32_v4rho2sigma2_11_chunk911(t11233: f64, t11278: f64, t209: f64, t3655: f64, t575: f64, t687: f64, t1049: f64, t8598: f64, t2967: f64, t8601: f64, t2964: f64, t3179: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11279 = t11233 + t11278;
    let t11280 = t11279 * t209;
    let t11281 = t3655 * t575;
    let t11282 = t11281 * t687;
    let t11283 = t8598 * t1049;
    let t11284 = 2.0_f64 * t11283;
    let t11285 = t8601 * t2967;
    let t11286 = 4.0_f64 * t11285;
    let t11287 = t2964 * t3179;
    (t11279, t11280, t11281, t11282, t11283, t11284, t11285, t11286, t11287)
}
