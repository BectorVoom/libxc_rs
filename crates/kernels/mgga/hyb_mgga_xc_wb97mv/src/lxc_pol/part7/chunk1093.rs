//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1093/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1093<F: Float>(t7: F, t1012: F, t4492: F, t1041: F, t4510: F, t1046: F, t3864: F, t7710: F, t2791: F, t3854: F, t10273: F, t1875: F, t224: F, t3641: F, t544: F, t3979: F, t7721: F, zeta_threshold: F) -> (F, F, F, F, F, F, F) {
    let t8 = t7 <= zeta_threshold;
    let t11580 = t1012 * t4492;
    let t11582 = t1041 * t4510;
    let t11584 = t1046 * t4510;
    let t11586 = t7710 * t3864;
    let t11591 = t2791 * t3854;
    let t11597 = piecewise3(t8, 0.0, -8.0 / 27.0 * t11586 * t544 + 16.0 / 9.0 * t3641 * t1875 + 4.0 / 9.0 * t11591 * t544 + 4.0 / 3.0 * t224 * t10273);
    let t11598 = t7721 * t3979;
    (t11580, t11582, t11584, t11586, t11591, t11597, t11598)
}
