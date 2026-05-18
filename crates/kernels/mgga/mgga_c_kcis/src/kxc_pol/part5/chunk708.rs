//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 708/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk708<F: Float>(t1684: F, t187: F, t1233: F, t1694: F, t3600: F, t4684: F, t4687: F, t4689: F, t4692: F, t4721: F, t4725: F, t4732: F, t4741: F, t4760: F, t4765: F, t5257: F, t972: F) -> (F, F) {
    let t5261 = t187 * t1684;
    let t5272 = -t4684 + t4687 + t4689 - t4692 + t4721 + t4725 + t187 * t5257 + F::new(0.19751789702565206229e-1) * t187 * t4732 - F::new(0.58482233974552040708e0) * t5261 * t972 - F::new(0.58482233974552040708e0) * t3600 * t1694 + F::new(0.11696446794910408142e1) * t1233 * t4741 - F::new(0.58482233974552040708e0) * t1233 * t4760 - F::new(0.17315755899375863299e2) * t1233 * t4765;
    (t5261, t5272)
}
