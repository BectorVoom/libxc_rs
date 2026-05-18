//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 1046/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk1046<F: Float>(t1097: F, t3372: F, t3376: F, t3422: F, t1128: F, t119: F, t841: F, t142: F, t2918: F, t1071: F, t2900: F, t3328: F) -> (F, F, F, F, F, F, F) {
    let t15492 = t1097 * t3372;
    let t15497 = t3422 * t3376;
    let t15498 = t15497 * t1128;
    let t15515 = t119 * t841;
    let t15522 = t142 * t2918;
    let t15526 = t119 * t1071;
    let t15537 = t142 * t2900;
    let t15541 = t142 * t3328;
    (t15492, t15498, t15515, t15522, t15526, t15537, t15541)
}
