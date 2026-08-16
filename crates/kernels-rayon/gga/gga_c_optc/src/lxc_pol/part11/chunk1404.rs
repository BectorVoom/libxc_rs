//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1404/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1404(t26262: f64, t26265: f64, t52395: f64, t58375: f64, t58378: f64, t58381: f64, t58384: f64, t58388: f64, t58392: f64, t58397: f64, t58401: f64, t58405: f64, t58409: f64, t58412: f64) -> f64 {
    let t59116 = -0.15944888888888888889e1_f64 * t52395 + t26262 + t26265 - 0.19931111111111111111e1_f64 * t58375 + 0.71752000000000000001e1_f64 * t58378 - 0.79724444444444444444e0_f64 * t58381 - 0.107628e2_f64 * t58384 + 0.43816888888888888889e0_f64 * t58388 + 0.49293999999999999999e0_f64 * t58392 + 0.39862222222222222223e1_f64 * t58397 - 0.71752000000000000002e1_f64 * t58401 - 0.59793333333333333333e0_f64 * t58405 + 0.71752e1_f64 * t58409 + 0.17938e1_f64 * t58412;
    t59116
}
