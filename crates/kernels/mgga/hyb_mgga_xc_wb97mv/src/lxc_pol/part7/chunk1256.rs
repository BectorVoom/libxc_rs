//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1256/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1256<F: Float>(t22498: F, t22501: F, t22725: F, t26298: F, t26301: F, t26304: F, t284: F, t30747: F, t30750: F, t30778: F, t26489: F, t9194: F, t26706: F, t9204: F, t3435: F, t3402: F) -> (F, F, F, F, F) {
    let t30792 = (t22725 - 0.57685185185185185184e-1 * t22498 + 0.12361111111111111111e-1 * t22501 - 0.57685185185185185187e-1 * t26298 + 0.49444444444444444446e-1 * t26301 - 0.18541666666666666667e-1 * t26304 + 0.12361111111111111111e-1 * t30747 - 0.18541666666666666667e-1 * t30750 + 0.278125e-1 * t30778) * t284;
    let t30795 = 24.0 * t26489 * t9194;
    let t30797 = 0.38596750796862084161e3 * t26706 * t9204;
    let t30801 = t3435 * t3435;
    let t30805 = t3402 * t3402;
    (t30792, t30795, t30797, t30801, t30805)
}
