//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1057/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1057<F: Float>(t1479: F, t60: F, t2122: F, t28150: F, t13272: F, t7565: F, t38: F, t8142: F, t2247: F, t116: F, t8151: F, t1450: F, t6816: F, t7237: F, t2014: F, t6836: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t29355 = t1479 * t60;
    let t29380 = t2122 * t28150;
    let t29388 = t13272 * t7565;
    let t29411 = t38 * t8142;
    let t29412 = t2247 * t29411;
    let t29427 = t8151 * t116;
    let t29494 = t1450 * t6816;
    let t29495 = t7237 * t29494;
    let t29497 = 3.0 * t2014 * t29495;
    let t29498 = t1450 * t6836;
    (t29355, t29380, t29388, t29411, t29412, t29427, t29494, t29495, t29497, t29498)
}
