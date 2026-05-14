//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 700/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk700<F: Float>(t784: F, t1425: F, t240: F, t2712: F, t136: F, t1412: F, t220: F, t4010: F, t72: F, t245: F, t1384: F, t138: F, t2438: F, t785: F, t2246: F, t599: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t9644 = t784 * t784;
    let t9645 = 1.0 / t9644;
    let t9655 = t1425 * t1425;
    let t9656 = 1.0 / t9655;
    let t9794 = t2712 * t240;
    let t9817 = t1412 * t136;
    let t9818 = t9817 * t220;
    let t9954 = t4010 * t72;
    let t9955 = t9954 * t245;
    let t9989 = t1384 * t1384;
    let t9990 = 1.0 / t9989;
    let t10073 = t138 * t2438 * t785;
    let t10301 = t599 * t2246;
    (t9644, t9645, t9655, t9656, t9794, t9818, t9955, t9990, t10073, t10301)
}
