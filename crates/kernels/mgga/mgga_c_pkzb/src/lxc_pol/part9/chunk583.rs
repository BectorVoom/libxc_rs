//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 583/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk583<F: Float>(t24: F, t1655: F, t2467: F, t422: F, t423: F, t960: F, t962: F, t330: F, t42: F, t448: F, t459: F, t987: F, t1424: F, t973: F, t440: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F, F, F, F, F, F) {
    let t90 = t24 <= zeta_threshold;
    let t332 = rho1 <= dens_threshold || t90;
    let t2471 = piecewise3(t90, 0.0, t1655);
    let t2475 = piecewise3(t332, 0.0, t2467 * t423 / 2.0 + t960 * t962 + t422 * t2471 / 2.0);
    let t2476 = t330 * t2475;
    let t2481 = t448 * t42;
    let t2484 = t987 * t459;
    let t2489 = t1424 * t973;
    let t2490 = t2489 * t440;
    (t2471, t2476, t2481, t2484, t2489, t2490)
}
