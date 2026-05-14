//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 875/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk875<F: Float>(t2664: F, t566: F, t222: F, t2668: F, t1019: F, t1846: F, t1037: F, t2631: F, t2634: F, t2689: F, t1078: F, t2746: F, t1063: F, t2702: F, t221: F, t450: F, t7509: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t7646 = t566 * t2664;
    let t7649 = 0.85917975471764868594e0 * t222 * t7646 * t2668;
    let t7650 = t1846 * t1019;
    let t7653 = 0.71233333333333333332e-1 * t222 * t7650 * t1037;
    let t7657 = 0.10685e0 * t222 * t566 * t2631 * t2634;
    let t7658 = t566 * t2689;
    let t7662 = t1846 * t1078;
    let t7669 = t566 * t2746;
    let t7673 = t1846 * t1063;
    let t7684 = t566 * t2702;
    let t7690 = 0.34450798614814814813e-2 * t221 * t7509 * t450;
    (t7646, t7649, t7650, t7653, t7657, t7658, t7662, t7669, t7673, t7684, t7690)
}
