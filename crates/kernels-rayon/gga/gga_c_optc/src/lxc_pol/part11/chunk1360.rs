//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1360/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1360(t27866: f64, t27867: f64, t52389: f64, t52391: f64, t52393: f64, t52395: f64, t58375: f64, t58378: f64, t58381: f64, t58384: f64, t58388: f64, t58392: f64) -> f64 {
    let t58394 = 0.25851111111111111111e1_f64 * t52389 + 0.15510666666666666667e2_f64 * t52391 + 0.28723456790123456789e1_f64 * t52393 - 0.10340444444444444444e2_f64 * t52395 + t27866 + t27867 - 0.12925555555555555555e2_f64 * t58375 + 0.46531999999999999998e2_f64 * t58378 - 0.5170222222222222222e1_f64 * t58381 - 0.69798e2_f64 * t58384 + 0.6568e-2_f64 * t58388 + 0.7389e-2_f64 * t58392;
    t58394
}
