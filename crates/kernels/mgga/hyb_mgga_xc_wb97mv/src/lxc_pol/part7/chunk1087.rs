//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1087/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1087<F: Float>(t3535: F, t9400: F, t4293: F, t956: F, t2517: F, t1404: F, t3530: F, t2474: F, t4322: F, t7376: F, t4319: F, t2519: F, t4318: F, t3534: F, t4292: F, t7405: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t11477 = 0.32163958997385070134e2 * t9400 * t3535;
    let t11478 = t4293 * t956;
    let t11480 = 6.0 * t2517 * t11478;
    let t11481 = t1404 * t3530;
    let t11483 = 4.0 * t2474 * t11481;
    let t11484 = t4322 * t956;
    let t11486 = 0.96491876992155210402e2 * t7376 * t11484;
    let t11487 = t4319 * t956;
    let t11489 = 2.0 * t2474 * t11487;
    let t11490 = t4318 * t2519;
    let t11491 = t11490 * t956;
    let t11493 = 0.16081979498692535067e2 * t2517 * t11491;
    let t11494 = t3534 * t3530;
    let t11496 = 0.32163958997385070134e2 * t2517 * t11494;
    let t11497 = t4292 * t7405;
    (t11477, t11478, t11480, t11481, t11483, t11484, t11486, t11487, t11489, t11490, t11491, t11493, t11494, t11496, t11497)
}
