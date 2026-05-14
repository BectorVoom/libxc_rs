//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 509/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk509<F: Float>(t7: F, t2170: F, t220: F, t2348: F, t291: F, t770: F, t860: F, t317: F, t322: F, t326: F, t1582: F, t328: F, t1815: F, t332: F, t897: F, t905: F, t298: F, t876: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F, F, F, F) {
    let t8 = t7 <= zeta_threshold;
    let t9 = rho0 <= dens_threshold || t8;
    let t2352 = piecewise3(t9, 0.0, t2170 * t291 / 2.0 + t770 * t860 + t220 * t2348 / 2.0);
    let t2356 = t322 * t317;
    let t2357 = t2356 * t326;
    let t2358 = t328 * t1582;
    let t2359 = t332 * t1815;
    let t2360 = t2358 * t2359;
    let t2363 = t897 * t905;
    let t2366 = t298 * t876;
    (t2352, t2357, t2360, t2363, t2366)
}
