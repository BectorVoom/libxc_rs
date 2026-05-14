//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 564/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk564<F: Float>(t2633: F, t2667: F, t2665: F, t1041: F, t1090: F, t1046: F, t1057: F, t2642: F, t2645: F, t2648: F, t2652: F, t2654: F, t2657: F) -> (F, F, F, F, F, F) {
    let t2668 = t2633 * t2667;
    let t2670 = 0.16081979498692535067e2 * t2665 * t2668;
    let t2672 = 8.0 * t1041 * t1090;
    let t2674 = 8.0 * t1046 * t1090;
    let t2675 = t1041 * t1057;
    let t2683 = -0.57538888888888888889e0 * t2642 + 0.11507777777777777778e1 * t2645 + 0.40256666666666666667e0 * t2648 + 0.366775e-1 * t2652 + 0.73355e-1 * t2654 + 0.137975e0 * t2657;
    (t2668, t2670, t2672, t2674, t2675, t2683)
}
