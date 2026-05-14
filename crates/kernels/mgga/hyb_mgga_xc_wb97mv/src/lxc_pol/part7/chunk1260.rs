//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1260/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1260<F: Float>(t2217: F, t30881: F, t22498: F, t22501: F, t22540: F, t26298: F, t26301: F, t26304: F, t30747: F, t30750: F, t30778: F, t788: F, t795: F, t2224: F, t238: F, t4184: F) -> (F, F, F, F) {
    let t30886 = t2217 * t30881;
    let t30901 = t22540 - 56.0 / 27.0 * t22498 + 4.0 / 9.0 * t22501 - 56.0 / 27.0 * t26298 + 16.0 / 9.0 * t26301 - 2.0 / 3.0 * t26304 + 4.0 / 9.0 * t30747 - 2.0 / 3.0 * t30750 + t30778;
    let t30902 = t788 * t30901;
    let t30904 = t795 * t30901;
    let t30907 = t238 * t2224 * t4184;
    (t30886, t30902, t30904, t30907)
}
