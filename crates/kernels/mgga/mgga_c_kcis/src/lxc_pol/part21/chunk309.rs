//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 309/1221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk309<F: Float>(t1207: F, t1212: F, t1221: F, t1225: F, t1226: F, t1233: F, t187: F, t405: F, t928: F, t953: F, t957: F, t972: F) -> (F,) {
    let t1236 = -t928 + t953 + t187 * (-0.3109e-1 * t1207 * t405 + 1.0 * t1212 * t1221 + t928 - t953 - 0.19751789702565206229e-1 * t957 + 0.58482233974552040708e0 * t1225 * t1226) + 0.19751789702565206229e-1 * t187 * t957 - 0.58482233974552040708e0 * t1233 * t972;
    (t1236,)
}
