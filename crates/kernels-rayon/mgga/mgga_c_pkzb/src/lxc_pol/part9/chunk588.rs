//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 588/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk588(t24: f64, t1655: f64, t2467: f64, t422: f64, t423: f64, t960: f64, t962: f64, t330: f64, t42: f64, t448: f64, t459: f64, t987: f64, t1424: f64, t973: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64) {
    let t90 = t24 <= zeta_threshold;
    let t332 = rho1 <= dens_threshold || t90;
    let t2471 = piecewise3(t90, 0.0_f64, t1655);
    let t2475 = piecewise3(t332, 0.0_f64, t2467 * t423 / 2.0_f64 + t960 * t962 + t422 * t2471 / 2.0_f64);
    let t2476 = t330 * t2475;
    let t2481 = t448 * t42;
    let t2484 = t987 * t459;
    let t2489 = t1424 * t973;
    (t2471, t2476, t2481, t2484, t2489)
}
