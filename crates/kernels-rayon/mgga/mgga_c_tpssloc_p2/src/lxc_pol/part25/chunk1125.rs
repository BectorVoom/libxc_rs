//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1125/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1125(t81446: f64, t9366: f64, t2358: f64, t666: f64, t22473: f64, t6530: f64, t9411: f64, t25014: f64, t9616: f64, t25373: f64, t46320: f64, t22960: f64, t46298: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t81447 = t81446 * t9366;
    let t81449 = t666 * t2358;
    let t81450 = t22473 * t81449;
    let t81452 = t6530 * t9411;
    let t81470 = t25014 * t9616;
    let t81476 = t25373 * t46320;
    let t81486 = t22960 * t46298;
    (t81447, t81450, t81452, t81470, t81476, t81486)
}
