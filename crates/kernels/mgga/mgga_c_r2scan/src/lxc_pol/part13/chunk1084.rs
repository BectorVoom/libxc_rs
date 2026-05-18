//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1084/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1084<F: Float>(t38145: F, t6085: F, t6087: F, t2161: F, t5148: F, t37638: F, t2111: F, t6461: F, t6072: F, t6064: F, t6093: F, t10698: F, t10805: F) -> (F, F, F, F, F, F, F) {
    let t38147 = t6085 * t38145 * t6087;
    let t38149 = t2161 * t5148;
    let t38150 = t38149 * t37638;
    let t38152 = t2111 * t6461;
    let t38153 = t38152 * t6072;
    let t38156 = t6093 * t38145 * t6064;
    let t38158 = t10698 * t10805;
    (t38147, t38149, t38150, t38152, t38153, t38156, t38158)
}
