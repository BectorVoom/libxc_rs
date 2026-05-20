//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2686/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2686<F: Float>(t11921: F, t19414: F, t247: F, t4837: F, t11710: F, t20078: F, t3091: F, t11922: F, t11927: F, t19621: F, t11774: F, t4787: F, t53391: F) -> (F, F, F, F) {
    let t67237 = t4837 * t247 * t11921 * t19414;
    let t67249 = t3091 * t11710 * t20078;
    let t67253 = t11927 * t11922 * t19621;
    let t67264 = t11774 * t53391 * t4787;
    (t67237, t67249, t67253, t67264)
}
