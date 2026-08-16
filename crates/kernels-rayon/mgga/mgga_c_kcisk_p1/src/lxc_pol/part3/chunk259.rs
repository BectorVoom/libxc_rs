//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 259/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk259(t1169: f64, t1194: f64, t1198: f64, t1201: f64, t1213: f64, t45: f64) -> f64 {
    let t1216 = -t1169 + t1194 + 0.19751789702565206229e-1_f64 * t45 * t1198 - 0.58482233974552040708e0_f64 * t1201 * t1213;
    t1216
}
