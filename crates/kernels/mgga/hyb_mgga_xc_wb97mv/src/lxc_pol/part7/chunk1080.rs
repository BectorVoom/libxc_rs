//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1080/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1080<F: Float>(t3494: F, t3531: F, t3621: F, t9564: F, t4359: F, t7315: F, t7318: F, t994: F, t11257: F, t11259: F, t11262: F, t11267: F, t11282: F, t11284: F, t11291: F, t11293: F, t7192: F, t7278: F, t9271: F, t9372: F) -> (F, F, F, F, F) {
    let t11342 = 2.0 * t3494 * t3531;
    let t11343 = t3621 * t9564;
    let t11346 = t7315 * t4359;
    let t11347 = t7318 * t994;
    let t11348 = t11346 * t11347;
    let t11363 = 0.19419375e1 * t11257 - 0.258925e1 * t11259 - 0.1294625e1 * t11262 + 0.258925e1 * t11284 - t7278 + 0.40256666666666666667e0 * t7192 + 0.80513333333333333333e0 * t9271 - t9372 - 0.301925e0 * t11267 + 0.905775e0 * t11282 - 0.412621875e-1 * t11291 + 0.16504875e0 * t11293;
    (t11342, t11343, t11346, t11348, t11363)
}
