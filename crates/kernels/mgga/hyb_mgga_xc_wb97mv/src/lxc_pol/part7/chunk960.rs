//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 960/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk960<F: Float>(t7: F, t9224: F, t9240: F, t1319: F, t1376: F, t2170: F, t220: F, t2348: F, t291: F, t3311: F, t3465: F, t770: F, t860: F, t8886: F, t336: F, t3469: F, t919: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F, F, F) {
    let t8 = t7 <= zeta_threshold;
    let t9 = rho0 <= dens_threshold || t8;
    let t9241 = t9224 + t9240;
    let t9245 = piecewise3(t9, 0.0, t8886 * t291 / 2.0 + t3311 * t860 + t1319 * t2348 / 2.0 + t2170 * t1376 / 2.0 + t770 * t3465 + t220 * t9241 / 2.0);
    let t9246 = t9245 * t336;
    let t9247 = t3469 * t919;
    (t9241, t9245, t9246, t9247)
}
