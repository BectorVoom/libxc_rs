//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 661/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk661<F: Float>(t3333: F, t810: F, t1341: F, t2194: F, t808: F, t2199: F, t1330: F, t2205: F, t790: F, t2178: F, t2209: F, t3317: F, t3328: F) -> (F, F, F, F, F, F, F) {
    let t3335 = 1.0 * t3333 * t810;
    let t3337 = 1.0 * t2194 * t1341;
    let t3338 = t1341 * t808;
    let t3340 = 2.0 * t2199 * t3338;
    let t3341 = t2205 * t1330;
    let t3342 = t3341 * t790;
    let t3346 = t2209 - t2178 / 3.0 - t3317 / 3.0 + t3328;
    (t3335, t3337, t3338, t3340, t3341, t3342, t3346)
}
