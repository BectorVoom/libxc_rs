//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 713/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk713<F: Float>(t10568: F, t10641: F, t10579: F, t10582: F, t10590: F, t10598: F, t10644: F, t10647: F, t10661: F, t10665: F, t10667: F, t10672: F, t10675: F, t10678: F, t12037: F, t1974: F) -> (F,) {
    let t12042 = 0.16068111111111111111e1 * t10568;
    let t12043 = 0.46308888888888888888e0 * t10641;
    let t12052 = -0.57386111111111111112e0 * t10579 + 0.20659e1 * t10582 - 0.309885e1 * t10590 - 0.516475e0 * t10598 - t12042 - t12043 - 0.104195e0 * t10644 + 0.62517e0 * t10647 + 0.6311625e0 * t10661 - 0.157790625e0 * t10665 + 0.3529725e1 * t10667 + 0.264729375e1 * t10672 - 0.52945875e1 * t10675 + 0.94674375e0 * t10678;
    let t12053 = t12037 + t12052;
    let t12054 = t12053 * t1974;
    (t12054,)
}
