//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 965/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk965<F: Float>(t9310: F, t996: F, t260: F, t3574: F, t1393: F, t7266: F, t2481: F, t7282: F, t238: F, t3522: F, t800: F, t3526: F, t1386: F, t2462: F, t242: F, t7192: F, t7195: F, t7278: F, t7292: F, t7294: F, t7297: F, t7300: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t9311 = t9310 * t996;
    let t9318 = t260 * t3574;
    let t9334 = t7266 * t1393;
    let t9335 = t9334 * t2481;
    let t9337 = t7282 * t1393;
    let t9338 = t9337 * t2481;
    let t9341 = t238 * t800 * t3522;
    let t9342 = 0.33114e0 * t9341;
    let t9344 = t238 * t800 * t3526;
    let t9345 = 0.33114e0 * t9344;
    let t9346 = t2462 * t1386;
    let t9348 = t238 * t242 * t9346;
    let t9350 = 0.80513333333333333334e0 * t7192 - 0.301925e0 * t7195 - t7292 + 0.5519e0 * t7294 - 0.16557e0 * t7297 - 0.16557e0 * t7300 - t7278 + 0.19419375e1 * t9335 - 0.412621875e-1 * t9338 - t9342 - t9345 + 0.248355e0 * t9348;
    (t9311, t9318, t9334, t9335, t9337, t9338, t9341, t9342, t9344, t9345, t9346, t9348, t9350)
}
