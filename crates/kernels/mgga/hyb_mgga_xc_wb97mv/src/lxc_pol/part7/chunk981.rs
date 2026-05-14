//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 981/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk981<F: Float>(t132: F, t9399: F, t9605: F, t1008: F, t1382: F, t1439: F, t2445: F, t2620: F, t338: F, t3472: F, t3626: F, t392: F, t921: F, t9251: F, t432: F, t1498: F, t2624: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F, F, F, F) {
    let t133 = t132 <= zeta_threshold;
    let t134 = rho1 <= dens_threshold || t133;
    let t9606 = t9399 + t9605;
    let t9610 = piecewise3(t134, 0.0, t9251 * t392 / 2.0 + t3472 * t1008 + t1382 * t2620 / 2.0 + t2445 * t1439 / 2.0 + t921 * t3626 + t338 * t9606 / 2.0);
    let t9611 = t9610 * t432;
    let t9612 = t2624 * t1498;
    (t9606, t9610, t9611, t9612)
}
