//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 579/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk579<F: Float>(t1070: F, t2731: F, t2642: F, t2645: F, t2648: F, t2652: F, t2654: F, t2657: F) -> (F, F) {
    let t2732 = t2731 * t1070;
    let t2741 = -0.78438333333333333333e0 * t2642 + 0.15687666666666666667e1 * t2645 + 0.68863333333333333333e0 * t2648 + 0.14025833333333333333e0 * t2652 + 0.28051666666666666667e0 * t2654 + 0.17365833333333333333e0 * t2657;
    (t2732, t2741)
}
