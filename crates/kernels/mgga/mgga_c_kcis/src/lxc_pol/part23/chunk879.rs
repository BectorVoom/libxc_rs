//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 879/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk879<F: Float>(t15844: F, t1354: F, t6114: F, t2084: F, t3938: F, t3919: F, t6117: F, t3947: F, t5613: F, t11539: F, t1919: F, t1911: F, t3944: F, t12736: F, t12741: F, t12744: F, t12751: F, t12755: F, t16092: F, t4366: F, t4370: F, t4373: F, t4374: F, t6106: F, t6111: F, t6118: F) -> (F, F) {
    let t17739 = 0.15476481481481481481e-2 * t15844;
    let t17762 = t6114 * t1354;
    let t17765 = t2084 * t3938;
    let t17768 = t6117 * t3919;
    let t17771 = t5613 * t3947;
    let t17772 = t17771 * t1354;
    let t17775 = t6117 * t3938;
    let t17778 = t1919 * t11539;
    let t17779 = t17778 * t3919;
    let t17784 = t1911 * t3944;
    let t17789 = -0.23392893589820816284e1 * t12755 * t6111 + 0.34631511798751726598e2 * t12741 * t6118 - 0.23392893589820816284e1 * t4366 * t17762 - 0.11696446794910408142e1 * t4366 * t17765 - 0.1038945353962551798e3 * t12744 * t17768 + 0.34631511798751726598e2 * t4373 * t17772 + 0.17315755899375863299e2 * t4373 * t17775 + 0.1025389702100779493e4 * t12751 * t17779 + t16092 + 0.58482233974552040708e0 * t6106 * t4370 + 0.17315755899375863299e2 * t17784 * t4374 + 0.58482233974552040708e0 * t12736 * t2084;
    (t17739, t17789)
}
