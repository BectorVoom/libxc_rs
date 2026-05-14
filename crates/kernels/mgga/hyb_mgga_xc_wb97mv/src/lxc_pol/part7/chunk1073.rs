//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1073/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1073<F: Float>(t7: F, t11038: F, t11235: F, t10907: F, t1319: F, t1376: F, t220: F, t291: F, t3311: F, t3465: F, t4143: F, t4267: F, t770: F, t860: F, t336: F, t4271: F, t919: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F, F, F) {
    let t8 = t7 <= zeta_threshold;
    let t9 = rho0 <= dens_threshold || t8;
    let t11236 = t11038 + t11235;
    let t11240 = piecewise3(t9, 0.0, t10907 * t291 / 2.0 + t4143 * t860 / 2.0 + t3311 * t1376 + t1319 * t3465 + t770 * t4267 / 2.0 + t220 * t11236 / 2.0);
    let t11241 = t11240 * t336;
    let t11242 = t4271 * t919;
    (t11236, t11240, t11241, t11242)
}
