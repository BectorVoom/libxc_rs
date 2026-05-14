//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 702/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk702<F: Float>(t132: F, t1008: F, t1382: F, t1439: F, t338: F, t3472: F, t3626: F, t392: F, t921: F, t1041: F, t1508: F, t1046: F, t1507: F, t479: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F, F, F, F) {
    let t133 = t132 <= zeta_threshold;
    let t134 = rho1 <= dens_threshold || t133;
    let t3630 = piecewise3(t134, 0.0, t1382 * t1008 / 2.0 + t921 * t1439 / 2.0 + t338 * t3626 / 2.0 + t3472 * t392 / 2.0);
    let t3633 = t1041 * t1508;
    let t3635 = t1046 * t1508;
    let t3638 = t1507 * t479;
    (t3630, t3633, t3635, t3638)
}
