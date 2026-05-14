//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 267/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk267<F: Float>(t315: F, t964: F, t902: F, t928: F, t908: F, t919: F, t924: F, t932: F) -> (F, F) {
    let t965 = t315 * t964;
    let t967 = 0.301925e0 * t902;
    let t970 = 0.82785e-1 * t928;
    let t972 = 0.258925e1 * t919 - t967 - 0.301925e0 * t908 + 0.16504875e0 * t924 - t970 - 0.82785e-1 * t932;
    (t965, t972)
}
