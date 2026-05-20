//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 672/1507 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk672<F: Float>(t1046: F, t3172: F, t1041: F, t1066: F, t2862: F, t247: F, t283: F, t905: F) -> (F, F, F, F) {
    let t3173 = t3172 * t1046;
    let t3174 = t1041 * t3173;
    let t3176 = t1066 * t2862;
    let t3177 = t247 * t3176;
    let t3181 = F::new(1.0) / t283 / t905;
    (t3173, t3174, t3177, t3181)
}
