//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 1086/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk1086(t9185: f64, t9191: f64, t9195: f64, t9199: f64, t9202: f64, t9207: f64, t9214: f64, t9227: f64, t9232: f64, t9234: f64, t9236: f64, t9238: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t42345 = 0.25538759935978703638e-4_f64 * t9185;
    let t42346 = 0.51077519871957407276e-4_f64 * t9191;
    let t42347 = 0.76616279807936110914e-4_f64 * t9195;
    let t42348 = 0.25538759935978703638e-4_f64 * t9199;
    let t42349 = 0.25538759935978703638e-4_f64 * t9202;
    let t42350 = 0.31923449919973379548e-4_f64 * t9207;
    let t42351 = 0.17025839957319135759e-4_f64 * t9214;
    let t42355 = 0.4726e1_f64 * t9227;
    let t42356 = 0.4726e1_f64 * t9232;
    let t42357 = 0.4726e1_f64 * t9234;
    let t42358 = 0.85129199786595678796e-5_f64 * t9236;
    let t42359 = 0.11974241701863808564e0_f64 * t9238;
    (t42345, t42346, t42347, t42348, t42349, t42350, t42351, t42355, t42356, t42357, t42358, t42359)
}
