//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 886/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk886<F: Float>(t1130: F, t15176: F, t15179: F, t15181: F, t15183: F, t15187: F, t15191: F, t15195: F, t15198: F, t15212: F, t15214: F, t15226: F, t15484: F, t1097: F, t3372: F, t3376: F, t3422: F) -> (F, F, F) {
    let t15488 = -0.2089325e-1 * t15176 - 0.2089325e-1 * t15179 + 0.55715333333333333331e-1 * t15181 + 0.27857666666666666666e-1 * t15183 - 0.41786499999999999999e-1 * t15187 + 0.69644166666666666665e-2 * t15191 + 0.65001222222222222219e-1 * t15195 - 0.65001222222222222219e-1 * t15198 + 0.41786499999999999999e-1 * t15212 - 0.55715333333333333331e-1 * t15214 - 0.579e0 * t15484 * t1130 - 0.72223580246913580243e-1 * t15226;
    let t15492 = t1097 * t3372;
    let t15497 = t3422 * t3376;
    (t15488, t15492, t15497)
}
