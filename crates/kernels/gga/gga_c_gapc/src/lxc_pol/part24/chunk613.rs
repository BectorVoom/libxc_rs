//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 613/1133 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk613<F: Float>(t1552: F, t1672: F, t1338: F, t1971: F, t198: F, t1037: F, t457: F, t505: F, t1689: F, t567: F, t147: F, t1601: F, t1698: F, t442: F, t619: F, t681: F) -> (F, F, F, F, F, F, F, F, F) {
    let t5075 = t1672 * t1552;
    let t5079 = t1672 * t1338;
    let t5116 = t1971 * t198;
    let t5117 = t1037 * t457;
    let t5121 = t1037 * t505;
    let t5126 = t1689 * t567;
    let t5144 = t1601 * t147;
    let t5189 = t1698 * t442;
    let t5190 = t619 * t5189;
    let t5199 = t681 * t457;
    (t5075, t5079, t5116, t5117, t5121, t5126, t5144, t5190, t5199)
}
