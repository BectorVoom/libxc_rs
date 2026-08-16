//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1380/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1380<F: Float>(t1094: F, t1102: F, t58311: F, t8749: F, t17348: F, t4299: F, t4300: F, t58348: F, t58375: F, t58378: F, t58381: F, t58384: F, t58397: F, t58401: F, t58405: F, t58409: F, t58412: F, t58431: F) -> (F, F, F) {
    let t58591 = F::cast_from(0.1403573615389248977e2_f64) * t1102 * t8749 * t58311 * t1094;
    let t58596 = t4299 * t4300 * t17348;
    let t58614 = F::cast_from(0.23744444444444444444e0_f64) * t58397 - F::cast_from(0.11872222222222222222e0_f64) * t58375 - F::cast_from(0.42739999999999999999e0_f64) * t58401 + F::cast_from(0.42739999999999999999e0_f64) * t58378 - F::cast_from(0.35616666666666666666e-1_f64) * t58405 - F::cast_from(0.47488888888888888888e-1_f64) * t58381 + F::cast_from(0.4274e0_f64) * t58409 - F::cast_from(0.6411e0_f64) * t58384 + F::cast_from(0.10685e0_f64) * t58412 + F::cast_from(0.14246666666666666667e0_f64) * t58348 - F::cast_from(0.52765432098765432099e-1_f64) * t58431;
    (t58591, t58596, t58614)
}
