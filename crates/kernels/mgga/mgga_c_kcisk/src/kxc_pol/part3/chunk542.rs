//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 542/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk542<F: Float>(t4788: F, t4790: F, t1674: F, t1686: F, t45: F, t4698: F, t4701: F, t4708: F, t4739: F, t4747: F, t4754: F, t4757: F, t4764: F, t4783: F) -> (F, F) {
    let t4791 = t4788 * t4790;
    let t4794 = -t4698 + t4701 - t4708 + t4739 + t4747 + 0.19751789702565206229e-1 * t45 * t4754 - 0.11696446794910408142e1 * t4757 * t1686 + 0.11696446794910408142e1 * t1674 * t4764 - 0.58482233974552040708e0 * t1674 * t4783 - 0.17315755899375863299e2 * t1674 * t4791;
    (t4791, t4794)
}
