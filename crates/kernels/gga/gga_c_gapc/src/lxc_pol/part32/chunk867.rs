//! GGA_C_GAPC lxc pol — lxc_pol part 32 (v4rho2sigma2_11) CSE chunk 867/1129 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part32_v4rho2sigma2_11_chunk867<F: Float>(t3757: F, t920: F, t129: F, t7073: F, t3284: F, t7200: F, t11387: F, t2580: F, t7204: F, t11483: F, t933: F, t2597: F, t7735: F, t277: F, t655: F, t2763: F, t3137: F, t327: F, t7191: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t11739 = t3757 * t920;
    let t11741 = t7073 * t129;
    let t11742 = t3284 * t7200;
    let t11743 = t11741 * t11742;
    let t11745 = t11387 * t2580;
    let t11746 = t7204 * t11745;
    let t11748 = t933 * t11483;
    let t11749 = t2597 * t7735;
    let t11750 = t11748 * t11749;
    let t11752 = t277 * t655;
    let t11755 = t3137 * t327 * t2763 * t7191;
    (t11739, t11741, t11742, t11743, t11745, t11746, t11748, t11749, t11750, t11752, t11755)
}
