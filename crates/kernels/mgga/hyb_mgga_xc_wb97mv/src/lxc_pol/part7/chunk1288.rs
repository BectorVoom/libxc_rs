//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1288/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1288<F: Float>(t7: F, t10907: F, t11236: F, t1319: F, t1376: F, t2170: F, t220: F, t2348: F, t291: F, t30728: F, t31438: F, t31470: F, t31485: F, t31494: F, t31497: F, t31498: F, t31524: F, t31556: F, t3311: F, t3465: F, t4143: F, t4267: F, t770: F, t860: F, t8886: F, t9241: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F,) {
    let t8 = t7 <= zeta_threshold;
    let t9 = rho0 <= dens_threshold || t8;
    let t31563 = piecewise3(t9, 0.0, t30728 * t291 / 2.0 + t10907 * t860 + t4143 * t2348 / 2.0 + t8886 * t1376 + 2.0 * t3311 * t3465 + t1319 * t9241 + t2170 * t4267 / 2.0 + t770 * t11236 + t220 * (t31438 + t31470 + t31485 + t31494 + t31497 + t31498 + t31524 + t31556) / 2.0);
    (t31563,)
}
