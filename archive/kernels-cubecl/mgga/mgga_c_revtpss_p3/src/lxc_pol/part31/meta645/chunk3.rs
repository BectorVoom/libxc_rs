//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2108/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2108<F: Float>(t29643: F, t686: F, t72: F, t93281: F, t93317: F, t18451: F, t25270: F, t18462: F, t18647: F, t18527: F, t98988: F, t18471: F) -> (F, F, F, F, F, F, F) {
    let t105973 = t29643 * t72 * t686;
    let t105974 = t93281 * t105973;
    let t105976 = t93317 * t105973;
    let t105985 = t25270 * t18451;
    let t105987 = t25270 * t18462;
    let t105989 = t25270 * t18647;
    let t105991 = t98988 * t18527;
    let t105993 = t25270 * t18471;
    (t105974, t105976, t105985, t105987, t105989, t105991, t105993)
}
