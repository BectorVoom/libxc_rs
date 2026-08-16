//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 521/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk521(t4758: f64, t962: f64, t971: f64, t1692: f64, t3031: f64, t3034: f64, t969: f64, t1694: f64, t3001: f64, t45: f64, t4684: f64, t4687: f64, t4689: f64, t4692: f64, t4721: f64, t4725: f64, t4732: f64, t4735: f64, t4741: f64, t960: f64, t972: f64) -> (f64, f64, f64, f64, f64) {
    let t4760 = t962 * t4758 * t971;
    let t4763 = t3031 * t1692;
    let t4764 = t3034 * t969;
    let t4765 = t4763 * t4764;
    let t4768 = -t4684 + t4687 + t4689 - t4692 + t4721 + t4725 + 0.19751789702565206229e-1_f64 * t45 * t4732 - 0.58482233974552040708e0_f64 * t4735 * t972 - 0.58482233974552040708e0_f64 * t3001 * t1694 + 0.11696446794910408142e1_f64 * t960 * t4741 - 0.58482233974552040708e0_f64 * t960 * t4760 - 0.17315755899375863299e2_f64 * t960 * t4765;
    (t4760, t4763, t4764, t4765, t4768)
}
