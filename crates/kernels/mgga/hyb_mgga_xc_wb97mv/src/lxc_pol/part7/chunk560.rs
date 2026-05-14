//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 560/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk560<F: Float>(t221: F, t2647: F, t435: F, t236: F, t2641: F, t1029: F, t2644: F, t14: F, t2223: F, t237: F, t2642: F, t2645: F) -> (F, F, F, F, F, F) {
    let t2648 = t221 * t2647;
    let t2650 = 1.0/f64::sqrt(t435);
    let t2651 = t2650 * t236;
    let t2652 = t2651 * t2641;
    let t2654 = t1029 * t2644;
    let t2657 = t237 * t14 * t2223;
    let t2659 = -0.42198333333333333333e0 * t2642 + 0.84396666666666666666e0 * t2645 + 0.39862222222222222223e0 * t2648 + 0.68258333333333333333e-1 * t2652 + 0.13651666666666666667e0 * t2654 + 0.13692777777777777778e0 * t2657;
    (t2648, t2651, t2652, t2654, t2657, t2659)
}
