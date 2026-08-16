//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1008/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1008(t10845: f64, t1233: f64, t13798: f64, t13801: f64, t13805: f64, t13812: f64, t13819: f64, t13823: f64, t13827: f64, t13854: f64, t13860: f64, t13866: f64, t13974: f64, t13977: f64, t14028: f64, t1694: f64, t187: f64, t3008: f64, t3035: f64, t3600: f64, t4760: f64, t5261: f64) -> f64 {
    let t15291 = -0.17315755899375863299e2_f64 * t1233 * t13812 + 0.23392893589820816284e1_f64 * t1233 * t13823 - 0.35089340384731224426e1_f64 * t1233 * t13860 - 0.1025389702100779493e4_f64 * t1233 * t13819 + 0.11696446794910408142e1_f64 * t1233 * t13977 - t13798 - t13801 + t13805 + 0.19751789702565206229e-1_f64 * t187 * t13974 + 0.1038945353962551798e3_f64 * t1233 * t13827 - 0.58482233974552040708e0_f64 * t10845 * t1694 - 0.11696446794910408142e1_f64 * t3600 * t4760 + 0.11696446794910408142e1_f64 * t5261 * t3008 - t13854 - 0.58482233974552040708e0_f64 * t1233 * t14028 - 0.17315755899375863299e2_f64 * t5261 * t3035 - t13866;
    t15291
}
