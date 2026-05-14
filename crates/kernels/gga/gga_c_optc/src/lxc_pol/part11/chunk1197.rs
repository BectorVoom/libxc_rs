//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1197/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1197<F: Float>(t49385: F, t49387: F, t49393: F, t49395: F, t57012: F, t57016: F, t57020: F, t57024: F, t57027: F, t57030: F, t57034: F, t39545: F, t39560: F, t39565: F, t49404: F, t49406: F, t57037: F, t57041: F, t57044: F, t57048: F, t57057: F, t57060: F, t57063: F) -> (F, F) {
    let t58156 = -0.38768888888888888889e4 * t49385 + 0.58153333333333333332e4 * t49387 + 0.96922222222222222224e3 * t49393 + 0.10769135802469135803e4 * t49395 + 1888.0 * t57012 + 0.47199999999999999999e3 * t57016 - 0.78666666666666666666e2 * t57020 - 17446.0 * t57024 - 0.14538333333333333333e4 * t57027 - 0.94399999999999999998e3 * t57030 + 0.41955555555555555555e3 * t57034;
    let t58169 = 0.96922222222222222221e4 * t57037 - 0.21538271604938271605e4 * t57041 - 0.81580246913580246914e2 * t57044 - 0.78666666666666666667e2 * t57048 - 0.17481481481481481482e3 * t39545 - 0.52444444444444444446e3 * t39560 + 0.10488888888888888889e4 * t39565 + 0.12586666666666666667e4 * t49404 - 0.41955555555555555556e3 * t49406 - 0.72691666666666666667e3 * t57057 + 17446.0 * t57060 + 0.43614999999999999999e4 * t57063;
    (t58156, t58169)
}
