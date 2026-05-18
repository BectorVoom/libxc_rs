//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 287/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk287<F: Float>(t906: F, t930: F, t141: F, t908: F, t919: F, t921: F, t924: F, t929: F) -> (F, F, F) {
    let t931 = t930 * t906;
    let t932 = t141 * t931;
    let t934 = F::new(0.1898925e1) * t919 - t921 - F::new(0.29896666666666666667e0) * t908 + F::new(0.3071625e0) * t924 - t929 - F::new(0.82156666666666666667e-1) * t932;
    (t931, t932, t934)
}
