//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1236/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1236<F: Float>(t17348: F, t4299: F, t4300: F, t58348: F, t58375: F, t58378: F, t58381: F, t58384: F, t58397: F, t58401: F, t58405: F, t58409: F, t58412: F, t58431: F, t26836: F, t33724: F, t43414: F, t43503: F, t43508: F, t52389: F, t52391: F, t52393: F, t52395: F, t52446: F, t52452: F, t58435: F) -> (F, F, F) {
    let t58596 = t4299 * t4300 * t17348;
    let t58614 = 0.23744444444444444444e0 * t58397 - 0.11872222222222222222e0 * t58375 - 0.42739999999999999999e0 * t58401 + 0.42739999999999999999e0 * t58378 - 0.35616666666666666666e-1 * t58405 - 0.47488888888888888888e-1 * t58381 + 0.4274e0 * t58409 - 0.6411e0 * t58384 + 0.10685e0 * t58412 + 0.14246666666666666667e0 * t58348 - 0.52765432098765432099e-1 * t58431;
    let t58626 = -0.17808333333333333333e-1 * t58435 + 0.23744444444444444444e-1 * t52389 + 0.14246666666666666667e0 * t52391 - 0.47488888888888888888e-1 * t43503 + 0.94977777777777777776e-1 * t43508 + 0.47488888888888888888e-1 * t52446 - 0.14246666666666666667e0 * t52452 + 0.73871604938271604937e-1 * t33724 + t26836 + 0.26382716049382716049e-1 * t52393 - 0.94977777777777777776e-1 * t52395 - 0.31659259259259259258e-1 * t43414;
    (t58596, t58614, t58626)
}
