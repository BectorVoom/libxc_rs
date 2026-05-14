//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 920/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk920<F: Float>(t7: F, t8183: F, t8443: F, t8478: F, t8515: F, t3156: F, t762: F, t2013: F, t3158: F, t13: F, t2986: F, t6733: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F, F, F) {
    let t8 = t7 <= zeta_threshold;
    let t9 = rho0 <= dens_threshold || t8;
    let t8518 = piecewise3(t9, 0.0, t8183 + t8443 + t8478 + t8515);
    let t8519 = t3156 * t762;
    let t8523 = t3158 * t2013;
    let t8528 = t6733 * t13 * t2986;
    (t8518, t8519, t8523, t8528)
}
