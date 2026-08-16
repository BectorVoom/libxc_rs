//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 690/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk690(t3945: f64, t3947: f64, t1345: f64, t1357: f64, t3855: f64, t3858: f64, t3865: f64, t3896: f64, t3904: f64, t3911: f64, t3914: f64, t3921: f64, t3940: f64, t45: f64) -> (f64, f64) {
    let t3948 = t3945 * t3947;
    let t3951 = -t3855 + t3858 - t3865 + t3896 + t3904 + 0.19751789702565206229e-1_f64 * t45 * t3911 - 0.11696446794910408142e1_f64 * t3914 * t1357 + 0.11696446794910408142e1_f64 * t1345 * t3921 - 0.58482233974552040708e0_f64 * t1345 * t3940 - 0.17315755899375863299e2_f64 * t1345 * t3948;
    (t3948, t3951)
}
