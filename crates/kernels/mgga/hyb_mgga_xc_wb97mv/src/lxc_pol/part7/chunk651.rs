//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 651/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk651<F: Float>(t1246: F, t154: F, t3205: F, t711: F, t157: F, t715: F, t160: F, t719: F, t163: F, t723: F, t166: F, t727: F, t169: F, t731: F, t2109: F, t735: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t3208 = t154 * t1246;
    let t3211 = t711 * t3205;
    let t3213 = t157 * t1246;
    let t3216 = t715 * t3205;
    let t3218 = t160 * t1246;
    let t3221 = t719 * t3205;
    let t3223 = t163 * t1246;
    let t3226 = t723 * t3205;
    let t3228 = t166 * t1246;
    let t3231 = t727 * t3205;
    let t3233 = t169 * t1246;
    let t3236 = t731 * t3205;
    let t3238 = t2109 * t1246;
    let t3241 = t735 * t3205;
    (t3208, t3211, t3213, t3216, t3218, t3221, t3223, t3226, t3228, t3231, t3233, t3236, t3238, t3241)
}
