//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 1019/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk1019<F: Float>(t11730: F, t2578: F, t3768: F, t761: F, t334: F, t11533: F, t277: F, t3781: F, t3757: F, t920: F, t129: F, t7073: F) -> (F, F, F, F, F, F, F) {
    let t11731 = t2578 * t11730;
    let t11733 = t761 * t3768;
    let t11734 = t11733 * t334;
    let t11736 = t277 * t11533;
    let t11737 = t11736 * t3781;
    let t11739 = t3757 * t920;
    let t11741 = t7073 * t129;
    (t11731, t11733, t11734, t11736, t11737, t11739, t11741)
}
