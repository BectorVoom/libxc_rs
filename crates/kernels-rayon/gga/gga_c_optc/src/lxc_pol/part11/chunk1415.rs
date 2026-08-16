//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1415/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1415(t26599: f64, t26600: f64, t52395: f64, t58375: f64, t58378: f64, t58381: f64, t58384: f64, t58388: f64, t58392: f64, t58397: f64, t58401: f64, t58405: f64, t58409: f64, t58412: f64) -> f64 {
    let t59294 = -0.27545333333333333332e1_f64 * t52395 + t26599 + t26600 - 0.34431666666666666667e1_f64 * t58375 + 0.123954e2_f64 * t58378 - 0.13772666666666666667e1_f64 * t58381 - 0.185931e2_f64 * t58384 + 0.55570666666666666666e0_f64 * t58388 + 0.62517e0_f64 * t58392 + 0.68863333333333333334e1_f64 * t58397 - 0.123954e2_f64 * t58401 - 0.103295e1_f64 * t58405 + 0.123954e2_f64 * t58409 + 0.309885e1_f64 * t58412;
    t59294
}
