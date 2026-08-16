//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1176/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1176<F: Float>(t25746: F, t3332: F, t7628: F, t27177: F, t6165: F, t24786: F, t24790: F, t7614: F, t10760: F, t25303: F, t6085: F, t38145: F, t6535: F, t8089: F) -> (F, F, F, F, F, F) {
    let t40114 = t7628 * t3332 * t25746;
    let t40117 = t6165 * t3332 * t27177;
    let t40120 = t6165 * t3332 * t24786;
    let t40123 = t7614 * t3332 * t24790;
    let t40128 = t6085 * t10760 * t25303;
    let t40131 = t6535 * t38145 * t8089;
    (t40114, t40117, t40120, t40123, t40128, t40131)
}
