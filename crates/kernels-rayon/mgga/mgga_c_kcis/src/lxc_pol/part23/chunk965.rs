//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 965/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk965(t12736: f64, t12741: f64, t12744: f64, t12751: f64, t12755: f64, t16092: f64, t17762: f64, t17765: f64, t17768: f64, t17772: f64, t17775: f64, t17779: f64, t17784: f64, t2084: f64, t4366: f64, t4370: f64, t4373: f64, t4374: f64, t6106: f64, t6111: f64, t6118: f64) -> f64 {
    let t17789 = -0.23392893589820816284e1_f64 * t12755 * t6111 + 0.34631511798751726598e2_f64 * t12741 * t6118 - 0.23392893589820816284e1_f64 * t4366 * t17762 - 0.11696446794910408142e1_f64 * t4366 * t17765 - 0.1038945353962551798e3_f64 * t12744 * t17768 + 0.34631511798751726598e2_f64 * t4373 * t17772 + 0.17315755899375863299e2_f64 * t4373 * t17775 + 0.1025389702100779493e4_f64 * t12751 * t17779 + t16092 + 0.58482233974552040708e0_f64 * t6106 * t4370 + 0.17315755899375863299e2_f64 * t17784 * t4374 + 0.58482233974552040708e0_f64 * t12736 * t2084;
    t17789
}
