//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 658/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk658(t1684: f64, t187: f64, t1233: f64, t1694: f64, t3600: f64, t4684: f64, t4687: f64, t4689: f64, t4692: f64, t4721: f64, t4725: f64, t4732: f64, t4741: f64, t4760: f64, t4765: f64, t5257: f64, t972: f64) -> (f64, f64) {
    let t5261 = t187 * t1684;
    let t5272 = -t4684 + t4687 + t4689 - t4692 + t4721 + t4725 + t187 * t5257 + 0.19751789702565206229e-1_f64 * t187 * t4732 - 0.58482233974552040708e0_f64 * t5261 * t972 - 0.58482233974552040708e0_f64 * t3600 * t1694 + 0.11696446794910408142e1_f64 * t1233 * t4741 - 0.58482233974552040708e0_f64 * t1233 * t4760 - 0.17315755899375863299e2_f64 * t1233 * t4765;
    (t5261, t5272)
}
