//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 305/1348 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk305<F: Float>(t902: F, t928: F, t908: F, t919: F, t924: F, t932: F) -> (F, F, F) {
    let t967 = 0.301925e0 * t902;
    let t970 = 0.82785e-1 * t928;
    let t972 = 0.258925e1 * t919 - t967 - 0.301925e0 * t908 + 0.16504875e0 * t924 - t970 - 0.82785e-1 * t932;
    (t967, t970, t972)
}
