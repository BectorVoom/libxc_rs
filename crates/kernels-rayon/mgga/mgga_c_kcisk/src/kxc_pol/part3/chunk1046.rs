//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 1046/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk1046(t1097: f64, t3372: f64, t3376: f64, t3422: f64, t1128: f64, t119: f64, t841: f64, t142: f64, t2918: f64, t1071: f64, t2900: f64, t3328: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
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
