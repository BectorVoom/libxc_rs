//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 578/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk578<F: Float>(t2715: F, t492: F, t221: F, t2647: F, t450: F, t1063: F, t566: F, t1062: F, t471: F, t466: F, t1069: F) -> (F, F, F, F, F, F, F) {
    let t2717 = 32.0 * t2715 * t492;
    let t2720 = 0.14764627977777777777e-2 * t221 * t2647 * t450;
    let t2724 = t566 * t1063;
    let t2728 = t1062 * t471;
    let t2729 = 1.0 / t2728;
    let t2730 = t466 * t2729;
    let t2731 = t1069 * t1069;
    (t2717, t2720, t2724, t2728, t2729, t2730, t2731)
}
