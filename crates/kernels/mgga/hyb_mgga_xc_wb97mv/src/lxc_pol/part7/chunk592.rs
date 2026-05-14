//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 592/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk592<F: Float>(t2706: F, t2711: F, t2714: F, t2717: F, t2720: F, t2756: F, t2777: F, t2779: F, t2784: F, t2786: F, t2788: F, t2807: F, t2808: F, t458: F, t489: F, t2700: F) -> (F,) {
    let t2812 = t2706 + t2711 + t2714 - t2717 - t2720 + t2777 - 0.36622894612013090108e-3 * t2779 - t2756 + t2784 + 2.0 * t2786 - 8.0 * t2788 + t458 * t2808 + 0.19751673498613801407e-1 * t2807 * t489;
    let t2813 = t2700 + t2812;
    (t2813,)
}
