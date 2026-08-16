//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 937/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk937(t10989: f64, t11049: f64, t11002: f64, t1411: f64, t2480: f64, t294: f64, t3857: f64, t11004: f64, t10982: f64, t3819: f64, t876: f64, t1429: f64, t2574: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11172 = 0.11038e0_f64 * t10989;
    let t11179 = 0.22076e0_f64 * t11049;
    let t11188 = 0.13418888888888888889e0_f64 * t11002;
    let t11216 = t1411 * t2480;
    let t11222 = t294 * t3857;
    let t11276 = 0.2283111111111111111e-1_f64 * t11004;
    let t11277 = 0.11415555555555555555e-1_f64 * t10982;
    let t11289 = t3819 * t876;
    let t11294 = t1429 * t2574;
    (t11172, t11179, t11188, t11216, t11222, t11276, t11277, t11289, t11294)
}
