//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 780/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk780(t10568: f64, t10641: f64, t10579: f64, t10582: f64, t10590: f64, t10598: f64, t10644: f64, t10647: f64, t10661: f64, t10665: f64, t10667: f64, t10672: f64, t10675: f64, t10678: f64) -> f64 {
    let t12042 = 0.16068111111111111111e1_f64 * t10568;
    let t12043 = 0.46308888888888888888e0_f64 * t10641;
    let t12052 = -0.57386111111111111112e0_f64 * t10579 + 0.20659e1_f64 * t10582 - 0.309885e1_f64 * t10590 - 0.516475e0_f64 * t10598 - t12042 - t12043 - 0.104195e0_f64 * t10644 + 0.62517e0_f64 * t10647 + 0.6311625e0_f64 * t10661 - 0.157790625e0_f64 * t10665 + 0.3529725e1_f64 * t10667 + 0.264729375e1_f64 * t10672 - 0.52945875e1_f64 * t10675 + 0.94674375e0_f64 * t10678;
    t12052
}
