//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 153/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk153<F: Float>(t7: F, t132: F, t224: F, t22: F, t339: F, t259: F, zeta_threshold: F) -> (F, F, F, F) {
    let t8 = t7 <= zeta_threshold;
    let t133 = t132 <= zeta_threshold;
    let t459 = t224 * t7;
    let t460 = piecewise3(t8, t22, t459);
    let t461 = t339 * t132;
    let t462 = piecewise3(t133, t22, t461);
    let t463 = t460 + t462 - 2.0;
    let t464 = t463 * t259;
    (t459, t461, t463, t464)
}
