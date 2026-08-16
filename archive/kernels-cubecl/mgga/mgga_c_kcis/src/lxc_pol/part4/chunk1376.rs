//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1376/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1376<F: Float>(t12736: F, t12741: F, t12744: F, t12751: F, t12755: F, t16092: F, t17762: F, t17765: F, t17768: F, t17772: F, t17775: F, t17779: F, t17784: F, t2084: F, t4366: F, t4370: F, t4373: F, t4374: F, t6106: F, t6111: F, t6118: F) -> F {
    let t17789 = -F::cast_from(0.23392893589820816284e1_f64) * t12755 * t6111 + F::cast_from(0.34631511798751726598e2_f64) * t12741 * t6118 - F::cast_from(0.23392893589820816284e1_f64) * t4366 * t17762 - F::cast_from(0.11696446794910408142e1_f64) * t4366 * t17765 - F::cast_from(0.1038945353962551798e3_f64) * t12744 * t17768 + F::cast_from(0.34631511798751726598e2_f64) * t4373 * t17772 + F::cast_from(0.17315755899375863299e2_f64) * t4373 * t17775 + F::cast_from(0.1025389702100779493e4_f64) * t12751 * t17779 + t16092 + F::cast_from(0.58482233974552040708e0_f64) * t6106 * t4370 + F::cast_from(0.17315755899375863299e2_f64) * t17784 * t4374 + F::cast_from(0.58482233974552040708e0_f64) * t12736 * t2084;
    t17789
}
