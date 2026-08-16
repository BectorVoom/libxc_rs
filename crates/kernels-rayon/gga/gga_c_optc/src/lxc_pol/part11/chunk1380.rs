//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1380/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1380(t1094: f64, t1102: f64, t58311: f64, t8749: f64, t17348: f64, t4299: f64, t4300: f64, t58348: f64, t58375: f64, t58378: f64, t58381: f64, t58384: f64, t58397: f64, t58401: f64, t58405: f64, t58409: f64, t58412: f64, t58431: f64) -> (f64, f64, f64) {
    let t58591 = 0.1403573615389248977e2_f64 * t1102 * t8749 * t58311 * t1094;
    let t58596 = t4299 * t4300 * t17348;
    let t58614 = 0.23744444444444444444e0_f64 * t58397 - 0.11872222222222222222e0_f64 * t58375 - 0.42739999999999999999e0_f64 * t58401 + 0.42739999999999999999e0_f64 * t58378 - 0.35616666666666666666e-1_f64 * t58405 - 0.47488888888888888888e-1_f64 * t58381 + 0.4274e0_f64 * t58409 - 0.6411e0_f64 * t58384 + 0.10685e0_f64 * t58412 + 0.14246666666666666667e0_f64 * t58348 - 0.52765432098765432099e-1_f64 * t58431;
    (t58591, t58596, t58614)
}
