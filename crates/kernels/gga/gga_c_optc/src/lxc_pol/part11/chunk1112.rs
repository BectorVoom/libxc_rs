//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1112/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1112<F: Float>(t29330: F, t29335: F, t29341: F, t38368: F, t22581: F, t22593: F, t22697: F, t22703: F, t22708: F, t22711: F, t22716: F, t22719: F, t29346: F, t38375: F, t29348: F, t29350: F) -> (F, F, F, F, F, F, F, F, F) {
    let t56294 = 0.23392893589820816284e1 * t29330;
    let t56295 = 0.1926377843805564792e1 * t29335;
    let t56296 = 0.65061485296689145286e-1 * t29341;
    let t56297 = 48.0 * t38368;
    let t56298 = t22581 - t22593 - t56294 + t22697 + t22703 + t22708 - t22711 - t22716 - t22719 + t56295 + t56296 - t56297;
    let t56299 = 0.1403573615389248977e2 * t29346;
    let t56300 = 0.65061485296689145287e-1 * t38375;
    let t56301 = 0.86748647062252193714e-1 * t29348;
    let t56302 = 0.13012297059337829057e0 * t29350;
    (t56294, t56295, t56296, t56297, t56298, t56299, t56300, t56301, t56302)
}
