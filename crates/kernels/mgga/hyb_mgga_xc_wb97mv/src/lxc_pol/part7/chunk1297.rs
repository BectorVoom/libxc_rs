//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1297/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1297<F: Float>(t2517: F, t2519: F, t31750: F, t3531: F, t9444: F, t3494: F, t9441: F, t11306: F, t238: F, t800: F, t1847: F, t222: F, t4283: F) -> (F, F, F, F, F) {
    let t31763 = 0.32163958997385070134e2 * t2517 * t31750 * t2519;
    let t31767 = 4.0 * t9444 * t3531;
    let t31769 = 2.0 * t3494 * t9441;
    let t31771 = t238 * t800 * t11306;
    let t31779 = t222 * t1847 * t4283;
    (t31763, t31767, t31769, t31771, t31779)
}
