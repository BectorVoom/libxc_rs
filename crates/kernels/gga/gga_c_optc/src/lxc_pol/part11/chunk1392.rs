//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1392/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1392<F: Float>(t5434: F, t15562: F, t5264: F, t17469: F, t4305: F, t58348: F, t58375: F, t58378: F, t58381: F, t58384: F, t58397: F, t58401: F, t58405: F, t58409: F, t58412: F, t58431: F) -> (F, F, F, F) {
    let t58827 = t5434 * t5434;
    let t58834 = F::new(0.70178680769462448852e1) * t15562 * t5264;
    let t58836 = F::new(0.4155781415850207192e3) * t4305 * t17469;
    let t58848 = F::new(0.12361111111111111111e0) * t58397 - F::new(0.61805555555555555555e-1) * t58375 - F::new(0.22249999999999999999e0) * t58401 + F::new(0.22249999999999999999e0) * t58378 - F::new(0.18541666666666666666e-1) * t58405 - F::new(0.24722222222222222222e-1) * t58381 + F::new(0.2225e0) * t58409 - F::new(0.33375e0) * t58384 + F::new(0.55625000000000000001e-1) * t58412 + F::new(0.74166666666666666668e-1) * t58348 - F::new(0.27469135802469135803e-1) * t58431;
    (t58827, t58834, t58836, t58848)
}
