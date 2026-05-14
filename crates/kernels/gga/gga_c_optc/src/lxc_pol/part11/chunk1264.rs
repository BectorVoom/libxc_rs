//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1264/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1264<F: Float>(t58348: F, t58375: F, t58378: F, t58381: F, t58384: F, t58397: F, t58401: F, t58405: F, t58409: F, t58412: F, t58431: F, t26808: F, t33724: F, t43414: F, t43503: F, t43508: F, t52389: F, t52391: F, t52393: F, t52395: F, t52446: F, t52452: F, t58435: F) -> (F, F) {
    let t59392 = 0.2283111111111111111e0 * t58397 - 0.11415555555555555555e0 * t58375 - 0.41095999999999999999e0 * t58401 + 0.41095999999999999998e0 * t58378 - 0.34246666666666666665e-1 * t58405 - 0.4566222222222222222e-1 * t58381 + 0.41096e0 * t58409 - 0.61644e0 * t58384 + 0.10274e0 * t58412 + 0.13698666666666666667e0 * t58348 - 0.50735802469135802467e-1 * t58431;
    let t59404 = -0.17123333333333333333e-1 * t58435 + 0.22831111111111111111e-1 * t52389 + 0.13698666666666666667e0 * t52391 - 0.45662222222222222221e-1 * t43503 + 0.9132444444444444444e-1 * t43508 + 0.4566222222222222222e-1 * t52446 - 0.13698666666666666667e0 * t52452 + 0.71030123456790123454e-1 * t33724 + t26808 + 0.25367901234567901233e-1 * t52393 - 0.9132444444444444444e-1 * t52395 - 0.3044148148148148148e-1 * t43414;
    (t59392, t59404)
}
