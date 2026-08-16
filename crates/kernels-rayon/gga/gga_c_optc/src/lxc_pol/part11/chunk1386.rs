//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1386/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1386(t58348: f64, t58375: f64, t58378: f64, t58381: f64, t58384: f64, t58397: f64, t58401: f64, t58405: f64, t58409: f64, t58412: f64, t58431: f64, t26313: f64, t33724: f64, t43414: f64, t43503: f64, t43508: f64, t52389: f64, t52391: f64, t52393: f64, t52395: f64, t52446: f64, t52452: f64, t58435: f64) -> (f64, f64) {
    let t58740 = 40.0_f64 / 9.0_f64 * t58397 - 20.0_f64 / 9.0_f64 * t58375 - 8.0_f64 * t58401 + 8.0_f64 * t58378 - 2.0_f64 / 3.0_f64 * t58405 - 8.0_f64 / 9.0_f64 * t58381 + 8.0_f64 * t58409 - 12.0_f64 * t58384 + 2.0_f64 * t58412 + 8.0_f64 / 3.0_f64 * t58348 - 80.0_f64 / 81.0_f64 * t58431;
    let t58752 = -t58435 / 3.0_f64 + 4.0_f64 / 9.0_f64 * t52389 + 8.0_f64 / 3.0_f64 * t52391 - 8.0_f64 / 9.0_f64 * t43503 + 16.0_f64 / 9.0_f64 * t43508 + 8.0_f64 / 9.0_f64 * t52446 - 8.0_f64 / 3.0_f64 * t52452 + 112.0_f64 / 81.0_f64 * t33724 + t26313 + 40.0_f64 / 81.0_f64 * t52393 - 16.0_f64 / 9.0_f64 * t52395 - 16.0_f64 / 27.0_f64 * t43414;
    (t58740, t58752)
}
