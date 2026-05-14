//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 956/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk956<F: Float>(t2263: F, t3406: F, t2286: F, t3402: F, t827: F, t2278: F, t1352: F, t6905: F, t1341: F, t2200: F, t2245: F, t3338: F, t6937: F, t3374: F, t6914: F, t3370: F, t808: F) -> (F, F, F, F, F, F, F, F, F) {
    let t9173 = t3406 * t2263;
    let t9176 = t3402 * t2286;
    let t9177 = t9176 * t827;
    let t9180 = t3406 * t2278;
    let t9183 = t1352 * t6905;
    let t9184 = t9183 * t2263;
    let t9187 = t1341 * t2200;
    let t9189 = 6.0 * t2245 * t9187;
    let t9191 = 4.0 * t6937 * t3338;
    let t9193 = 0.32163958997385070134e2 * t6914 * t3374;
    let t9194 = t3370 * t808;
    (t9173, t9177, t9180, t9184, t9187, t9189, t9191, t9193, t9194)
}
