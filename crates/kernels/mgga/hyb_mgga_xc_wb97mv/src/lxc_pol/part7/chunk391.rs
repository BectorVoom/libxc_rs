//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 391/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk391<F: Float>(t1003: F, t1392: F, t1406: F, t1408: F, t1416: F, t1421: F, t1428: F, t1436: F, t260: F, t372: F, t968: F, t987: F) -> (F,) {
    let t1439 = -t1392 + t1406 + t260 * (-0.310907e-1 * t1408 * t372 + 1.0 * t968 * t1416 + t1392 - t1406 - 0.19751673498613801407e-1 * t1421 + 0.5848223622634646207e0 * t987 * t1428) + 0.19751673498613801407e-1 * t260 * t1421 - 0.5848223622634646207e0 * t1003 * t1436;
    (t1439,)
}
