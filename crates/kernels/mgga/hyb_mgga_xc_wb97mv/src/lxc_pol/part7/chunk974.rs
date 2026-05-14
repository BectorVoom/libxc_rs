//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 974/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk974<F: Float>(t1420: F, t2594: F, t9385: F, t995: F, t9341: F, t9344: F, t7192: F, t7195: F, t7294: F, t7297: F, t7300: F, t7341: F, t7348: F, t9335: F, t9338: F, t9348: F) -> (F, F, F, F, F) {
    let t9458 = t1420 * t2594;
    let t9465 = t9385 * t995;
    let t9475 = 0.41678e0 * t9341;
    let t9476 = 0.41678e0 * t9344;
    let t9478 = 0.13772666666666666667e1 * t7192 - 0.516475e0 * t7195 - t7348 + 0.69463333333333333333e0 * t7294 - 0.20839e0 * t7297 - 0.20839e0 * t7300 - t7341 + 0.264729375e1 * t9335 - 0.157790625e0 * t9338 - t9475 - t9476 + 0.312585e0 * t9348;
    (t9458, t9465, t9475, t9476, t9478)
}
