//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1029/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1029<F: Float>(t39841: F, t40033: F, t6087: F, t10752: F, t30370: F, t38145: F, t6085: F, t7922: F, t6093: F, t7605: F, t8081: F, t7619: F, t2147: F, t7624: F, t10719: F, t8198: F) -> (F, F, F, F, F, F, F, F) {
    let t40035 = t40033 * t39841 * t6087;
    let t40038 = t30370 * t10752;
    let t40041 = t6085 * t38145 * t7922;
    let t40044 = t6093 * t38145 * t7605;
    let t40047 = t6085 * t38145 * t8081;
    let t40050 = t6093 * t38145 * t7619;
    let t40053 = t2147 * t38145 * t7624;
    let t40059 = t8198 * t10719;
    (t40035, t40038, t40041, t40044, t40047, t40050, t40053, t40059)
}
