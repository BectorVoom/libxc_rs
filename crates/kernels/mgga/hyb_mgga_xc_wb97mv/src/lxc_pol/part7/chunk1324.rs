//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1324/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1324<F: Float>(t132: F, t1008: F, t11243: F, t11572: F, t1382: F, t1439: F, t2445: F, t2620: F, t31568: F, t31615: F, t31649: F, t31680: F, t31718: F, t31757: F, t31941: F, t31976: F, t32396: F, t338: F, t3472: F, t3626: F, t392: F, t4273: F, t4397: F, t921: F, t9251: F, t9606: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F,) {
    let t133 = t132 <= zeta_threshold;
    let t134 = rho1 <= dens_threshold || t133;
    let t32403 = piecewise3(t134, 0.0, t31568 * t392 / 2.0 + t11243 * t1008 + t4273 * t2620 / 2.0 + t9251 * t1439 + 2.0 * t3472 * t3626 + t1382 * t9606 + t2445 * t4397 / 2.0 + t921 * t11572 + t338 * (t31615 + t31649 + t31680 + t31718 + t31757 + t31941 + t31976 + t32396) / 2.0);
    (t32403,)
}
