//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1392/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1392(t5434: f64, t15562: f64, t5264: f64, t17469: f64, t4305: f64, t58348: f64, t58375: f64, t58378: f64, t58381: f64, t58384: f64, t58397: f64, t58401: f64, t58405: f64, t58409: f64, t58412: f64, t58431: f64) -> (f64, f64, f64, f64) {
    let t58827 = t5434 * t5434;
    let t58834 = 0.70178680769462448852e1_f64 * t15562 * t5264;
    let t58836 = 0.4155781415850207192e3_f64 * t4305 * t17469;
    let t58848 = 0.12361111111111111111e0_f64 * t58397 - 0.61805555555555555555e-1_f64 * t58375 - 0.22249999999999999999e0_f64 * t58401 + 0.22249999999999999999e0_f64 * t58378 - 0.18541666666666666666e-1_f64 * t58405 - 0.24722222222222222222e-1_f64 * t58381 + 0.2225e0_f64 * t58409 - 0.33375e0_f64 * t58384 + 0.55625000000000000001e-1_f64 * t58412 + 0.74166666666666666668e-1_f64 * t58348 - 0.27469135802469135803e-1_f64 * t58431;
    (t58827, t58834, t58836, t58848)
}
