//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 341/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk341(t1642: f64, t1667: f64, t1671: f64, t1674: f64, t1686: f64, t45: f64) -> f64 {
    let t1689 = -t1642 + t1667 + 0.19751789702565206229e-1_f64 * t45 * t1671 - 0.58482233974552040708e0_f64 * t1674 * t1686;
    t1689
}
