//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2177/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2177<F: Float>(t2470: F, t27278: F, t7064: F, t10073: F, t25402: F, t7056: F, t7759: F, t136: F, t2457: F, t7769: F, t93377: F, t4534: F, t689: F, t7014: F) -> (F, F, F, F, F, F) {
    let t99201 = t27278 * t2470;
    let t99202 = t7064 * t99201;
    let t99206 = t10073 * t7056 * t25402 * t7759;
    let t99211 = t7769 * t136 * t2457;
    let t99212 = t93377 * t99211;
    let t99216 = F::cast_from(0.10975748638225852664e-1_f64) * t689 * t7014 * t4534;
    (t99201, t99202, t99206, t99211, t99212, t99216)
}
