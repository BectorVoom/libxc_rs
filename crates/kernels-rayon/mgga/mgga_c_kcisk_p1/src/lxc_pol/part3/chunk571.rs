//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 571/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk571(t4788: f64, t4790: f64, t1674: f64, t1686: f64, t45: f64, t4698: f64, t4701: f64, t4708: f64, t4739: f64, t4747: f64, t4754: f64, t4757: f64, t4764: f64, t4783: f64) -> (f64, f64) {
    let t4791 = t4788 * t4790;
    let t4794 = -t4698 + t4701 - t4708 + t4739 + t4747 + 0.19751789702565206229e-1_f64 * t45 * t4754 - 0.11696446794910408142e1_f64 * t4757 * t1686 + 0.11696446794910408142e1_f64 * t1674 * t4764 - 0.58482233974552040708e0_f64 * t1674 * t4783 - 0.17315755899375863299e2_f64 * t1674 * t4791;
    (t4791, t4794)
}
