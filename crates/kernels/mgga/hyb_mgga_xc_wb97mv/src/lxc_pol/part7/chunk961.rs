//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 961/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk961<F: Float>(t132: F, t1380: F, t2443: F, t8885: F, t1427: F, t7315: F, t2574: F, t7318: F, t2575: F, t3621: F, t1386: F, t1847: F, t222: F, zeta_threshold: F) -> (F, F, F, F, F, F) {
    let t133 = t132 <= zeta_threshold;
    let t9249 = t1380 * t2443;
    let t9251 = piecewise3(t133, 0.0, -t8885);
    let t9260 = t7315 * t1427;
    let t9261 = t7318 * t2574;
    let t9262 = t9260 * t9261;
    let t9265 = t3621 * t2575;
    let t9271 = t222 * t1847 * t1386;
    (t9249, t9251, t9261, t9262, t9265, t9271)
}
