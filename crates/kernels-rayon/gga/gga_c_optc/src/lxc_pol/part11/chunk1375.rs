//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1375/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1375(t27950: f64, t27951: f64, t52389: f64, t52391: f64, t52393: f64, t52395: f64, t58375: f64, t58378: f64, t58381: f64, t58384: f64, t58388: f64, t58392: f64) -> f64 {
    let t58498 = 0.96922222222222222224e3_f64 * t52389 + 0.58153333333333333332e4_f64 * t52391 + 0.10769135802469135803e4_f64 * t52393 - 0.38768888888888888889e4_f64 * t52395 + t27950 + t27951 - 0.4846111111111111111e4_f64 * t58375 + 0.17445999999999999999e5_f64 * t58378 - 0.19384444444444444444e4_f64 * t58381 - 26169.0_f64 * t58384 + 0.41955555555555555555e3_f64 * t58388 + 0.47199999999999999999e3_f64 * t58392;
    t58498
}
