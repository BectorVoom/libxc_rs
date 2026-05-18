//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 690/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk690<F: Float>(t3945: F, t3947: F, t1345: F, t1357: F, t3855: F, t3858: F, t3865: F, t3896: F, t3904: F, t3911: F, t3914: F, t3921: F, t3940: F, t45: F) -> (F, F) {
    let t3948 = t3945 * t3947;
    let t3951 = -t3855 + t3858 - t3865 + t3896 + t3904 + F::new(0.19751789702565206229e-1) * t45 * t3911 - F::new(0.11696446794910408142e1) * t3914 * t1357 + F::new(0.11696446794910408142e1) * t1345 * t3921 - F::new(0.58482233974552040708e0) * t1345 * t3940 - F::new(0.17315755899375863299e2) * t1345 * t3948;
    (t3948, t3951)
}
