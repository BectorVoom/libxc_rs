//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1036/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1036<F: Float>(t7: F, t10238: F, t10278: F, t10294: F, t10541: F, t1283: F, t3156: F, t214: F, t3979: F, t674: F, t3: F, t3158: F, t1232: F, t1312: F, t3988: F, t2025: F, t4130: F, t683: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t8 = t7 <= zeta_threshold;
    let t9 = rho0 <= dens_threshold || t8;
    let t10544 = piecewise3(t9, 0.0, t10238 + t10278 + t10294 + t10541);
    let t10545 = t3156 * t1283;
    let t10549 = t214 * t3979;
    let t10550 = t10549 * t674;
    let t10554 = t3158 * t3;
    let t10558 = t1312 * t1232;
    let t10559 = t10558 * t674;
    let t10563 = t214 * t3988;
    let t10564 = t10563 * t674;
    let t10574 = t683 * t2025 * t4130;
    (t10544, t10545, t10549, t10550, t10554, t10558, t10559, t10563, t10564, t10574)
}
