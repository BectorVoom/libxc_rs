//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1232/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1232<F: Float>(t27950: F, t27951: F, t52389: F, t52391: F, t52393: F, t52395: F, t58375: F, t58378: F, t58381: F, t58384: F, t58388: F, t58392: F, t58397: F, t58401: F, t58405: F, t58409: F, t58412: F, t58415: F, t58418: F, t58421: F, t58424: F, t58428: F, t58431: F) -> (F, F) {
    let t58498 = 0.96922222222222222224e3 * t52389 + 0.58153333333333333332e4 * t52391 + 0.10769135802469135803e4 * t52393 - 0.38768888888888888889e4 * t52395 + t27950 + t27951 - 0.4846111111111111111e4 * t58375 + 0.17445999999999999999e5 * t58378 - 0.19384444444444444444e4 * t58381 - 26169.0 * t58384 + 0.41955555555555555555e3 * t58388 + 0.47199999999999999999e3 * t58392;
    let t58511 = 0.96922222222222222221e4 * t58397 - 17446.0 * t58401 - 0.14538333333333333333e4 * t58405 + 17446.0 * t58409 + 0.43614999999999999999e4 * t58412 - 0.78666666666666666667e2 * t58415 - 0.94399999999999999998e3 * t58418 - 0.78666666666666666666e2 * t58421 + 1888.0 * t58424 - 0.81580246913580246914e2 * t58428 - 0.21538271604938271605e4 * t58431;
    (t58498, t58511)
}
