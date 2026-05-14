//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1092/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1092<F: Float>(t132: F, t11544: F, t11571: F, t1008: F, t11243: F, t1382: F, t1439: F, t338: F, t3472: F, t3626: F, t392: F, t4273: F, t4397: F, t921: F, t432: F, t1498: F, t3630: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F, F, F, F) {
    let t133 = t132 <= zeta_threshold;
    let t134 = rho1 <= dens_threshold || t133;
    let t11572 = t11544 + t11571;
    let t11576 = piecewise3(t134, 0.0, t11243 * t392 / 2.0 + t4273 * t1008 / 2.0 + t3472 * t1439 + t1382 * t3626 + t921 * t4397 / 2.0 + t338 * t11572 / 2.0);
    let t11577 = t11576 * t432;
    let t11578 = t3630 * t1498;
    (t11572, t11576, t11577, t11578)
}
