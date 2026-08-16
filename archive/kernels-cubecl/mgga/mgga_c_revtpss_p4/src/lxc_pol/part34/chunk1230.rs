//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1230/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1230<F: Float>(t136: F, t2457: F, t7769: F, t93377: F, t2453: F, t27212: F, t25301: F, t25410: F, t7774: F, t93240: F, t7760: F, t786: F, t867: F) -> (F, F, F, F, F) {
    let t99211 = t7769 * t136 * t2457;
    let t99212 = t93377 * t99211;
    let t99257 = t2453 * t27212;
    let t99258 = t99257 * t25301;
    let t99261 = t93240 * t25410 * t7774;
    let t99285 = t786 * t7760 * t867;
    (t99211, t99212, t99258, t99261, t99285)
}
