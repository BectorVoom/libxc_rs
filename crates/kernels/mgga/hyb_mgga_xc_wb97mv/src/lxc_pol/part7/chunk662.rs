//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 662/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk662<F: Float>(t3346: F, t788: F, t1330: F, t2217: F, t790: F, t795: F, t1336: F, t238: F, t800: F) -> (F, F, F, F, F) {
    let t3347 = t788 * t3346;
    let t3352 = t2217 * t1330;
    let t3353 = t3352 * t790;
    let t3355 = t795 * t3346;
    let t3359 = t238 * t800 * t1336;
    (t3347, t3352, t3353, t3355, t3359)
}
