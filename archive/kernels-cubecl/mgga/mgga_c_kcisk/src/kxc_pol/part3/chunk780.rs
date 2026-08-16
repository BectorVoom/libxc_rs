//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 780/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk780<F: Float>(t10568: F, t10641: F, t10579: F, t10582: F, t10590: F, t10598: F, t10644: F, t10647: F, t10661: F, t10665: F, t10667: F, t10672: F, t10675: F, t10678: F) -> F {
    let t12042 = F::cast_from(0.16068111111111111111e1_f64) * t10568;
    let t12043 = F::cast_from(0.46308888888888888888e0_f64) * t10641;
    let t12052 = -F::cast_from(0.57386111111111111112e0_f64) * t10579 + F::cast_from(0.20659e1_f64) * t10582 - F::cast_from(0.309885e1_f64) * t10590 - F::cast_from(0.516475e0_f64) * t10598 - t12042 - t12043 - F::cast_from(0.104195e0_f64) * t10644 + F::cast_from(0.62517e0_f64) * t10647 + F::cast_from(0.6311625e0_f64) * t10661 - F::cast_from(0.157790625e0_f64) * t10665 + F::cast_from(0.3529725e1_f64) * t10667 + F::cast_from(0.264729375e1_f64) * t10672 - F::cast_from(0.52945875e1_f64) * t10675 + F::cast_from(0.94674375e0_f64) * t10678;
    t12052
}
