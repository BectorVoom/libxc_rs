//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1088/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1088<F: Float>(t11497: F, t956: F, t7403: F, t2558: F, t4345: F, t975: F, t3563: F, t3567: F, t11257: F, t11259: F, t11262: F, t11267: F, t11282: F, t11284: F, t11291: F, t11293: F, t7192: F, t7341: F, t9271: F, t9485: F) -> (F, F, F, F, F, F) {
    let t11498 = t11497 * t956;
    let t11500 = 0.51726012919273400301e3 * t7403 * t11498;
    let t11501 = t4345 * t2558;
    let t11502 = t11501 * t975;
    let t11505 = t3567 * t3563;
    let t11520 = 0.264729375e1 * t11257 - 0.3529725e1 * t11259 - 0.17648625e1 * t11262 + 0.3529725e1 * t11284 - t7341 + 0.68863333333333333333e0 * t7192 + 0.13772666666666666667e1 * t9271 - t9485 - 0.516475e0 * t11267 + 0.1549425e1 * t11282 - 0.157790625e0 * t11291 + 0.6311625e0 * t11293;
    (t11498, t11500, t11501, t11502, t11505, t11520)
}
