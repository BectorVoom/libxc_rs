//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1046/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1046<F: Float>(t263: F, t6660: F, t321: F, t6100: F, t1266: F, t818: F, t826: F, t11033: F, t1289: F, t11050: F, t6654: F, t1277: F, t3366: F, t6661: F) -> (F, F, F, F, F, F, F) {
    let t37031 = t263 * t6660;
    let t37038 = t6100 * t321;
    let t37040 = t1266 * t818;
    let t37041 = t37040 * t826;
    let t37043 = t11033 * t1289;
    let t37048 = t6654 * t11050;
    let t37055 = t6661 * t3366 * t1277;
    (t37031, t37038, t37040, t37041, t37043, t37048, t37055)
}
