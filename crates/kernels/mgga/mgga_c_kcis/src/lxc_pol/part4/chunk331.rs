//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 331/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk331<F: Float>(t1207: F, t1212: F, t1221: F, t1225: F, t1226: F, t1233: F, t187: F, t405: F, t928: F, t953: F, t957: F, t972: F) -> F {
    let t1236 = -t928 + t953 + t187 * (-F::new(0.3109e-1) * t1207 * t405 + F::new(1.0) * t1212 * t1221 + t928 - t953 - F::cast_from(0.19751789702565206229e-1_f64) * t957 + F::cast_from(0.58482233974552040708e0_f64) * t1225 * t1226) + F::cast_from(0.19751789702565206229e-1_f64) * t187 * t957 - F::cast_from(0.58482233974552040708e0_f64) * t1233 * t972;
    t1236
}
