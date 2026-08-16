//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1300/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1300(t25014: f64, t9616: f64, t25373: f64, t46320: f64, t193: f64, t201: f64, t6665: f64, t22960: f64, t46298: f64, t46252: f64, t46362: f64, t2249: f64, t776: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t81470 = t25014 * t9616;
    let t81476 = t25373 * t46320;
    let t81483 = t193 * t201 * t6665;
    let t81486 = t22960 * t46298;
    let t81489 = t22960 * t46252;
    let t81492 = t25373 * t46362;
    let t81501 = t2249 * t776;
    (t81470, t81476, t81483, t81486, t81489, t81492, t81501)
}
