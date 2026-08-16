//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 779/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk779<F: Float>(t4758: F, t962: F, t971: F, t1692: F, t3031: F, t3034: F, t969: F, t1694: F, t3001: F, t45: F, t4684: F, t4687: F, t4689: F, t4692: F, t4721: F, t4725: F, t4732: F, t4735: F, t4741: F, t960: F, t972: F) -> (F, F, F, F, F) {
    let t4760 = t962 * t4758 * t971;
    let t4763 = t3031 * t1692;
    let t4764 = t3034 * t969;
    let t4765 = t4763 * t4764;
    let t4768 = -t4684 + t4687 + t4689 - t4692 + t4721 + t4725 + F::cast_from(0.19751789702565206229e-1_f64) * t45 * t4732 - F::cast_from(0.58482233974552040708e0_f64) * t4735 * t972 - F::cast_from(0.58482233974552040708e0_f64) * t3001 * t1694 + F::cast_from(0.11696446794910408142e1_f64) * t960 * t4741 - F::cast_from(0.58482233974552040708e0_f64) * t960 * t4760 - F::cast_from(0.17315755899375863299e2_f64) * t960 * t4765;
    (t4760, t4763, t4764, t4765, t4768)
}
