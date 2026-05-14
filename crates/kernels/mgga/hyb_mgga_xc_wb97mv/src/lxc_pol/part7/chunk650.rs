//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 650/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk650<F: Float>(t143: F, t1232: F, t2033: F, t3194: F, t674: F, t3: F, t698: F, t701: F, t2058: F, t2059: F, t3040: F, t3186: F, t3191: F, t571: F) -> (F, F, F, F, F) {
    let t145 = 0.135e1 < t143;
    let t3195 = t2033 * t1232;
    let t3197 = t3194 * t3195 * t674;
    let t3201 = t698 * t701 * t3;
    let t3204 = t2058 + t2059 / 162.0 + t3186 / 162.0 - t571 * t3191 / 81.0 + t571 * t3197 / 27.0 + t3040 * t3201 / 27.0;
    let t3205 = piecewise3(t145, t3204, 0.0);
    (t3195, t3197, t3201, t3204, t3205)
}
