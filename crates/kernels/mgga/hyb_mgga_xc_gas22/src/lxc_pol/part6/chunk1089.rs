//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1089/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1089<F: Float>(t1123: F, t4576: F, t1129: F, t3663: F, t4851: F, t1134: F, t3760: F, t3767: F, t518: F, t1117: F, t3771: F, t3701: F, t9625: F, t1171: F, t4574: F, t3778: F, t510: F) -> (F, F, F, F, F, F, F, F, F) {
    let t11575 = t4576 * t1123;
    let t11578 = t4576 * t1129;
    let t11583 = t3663 * t4851;
    let t11586 = t1134 * t3760;
    let t11589 = t518 * t3767;
    let t11594 = t1117 * t3771;
    let t11597 = t9625 * t3701;
    let t11605 = t4574 * t1171;
    let t11608 = t510 * t3778;
    (t11575, t11578, t11583, t11586, t11589, t11594, t11597, t11605, t11608)
}
