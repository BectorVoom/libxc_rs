//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1693/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1693(t5: f64, t46123: f64, t117: f64, t10414: f64, t116: f64, t2319: f64, t2327: f64, t2371: f64, t112: f64, t46089: f64, t10199: f64, t666: f64, t2289: f64, t2341: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7 = piecewise3(0.0_f64 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0_f64;
    let t46124 = piecewise3(t8, 0.0_f64, t46123);
    let t46125 = t46124 * t117;
    let t46126 = t10414 * t116;
    let t46129 = t2319 * t2327;
    let t46137 = t2371 * t2371;
    let t46143 = 2618.0_f64 / 81.0_f64 * t46089 * t112;
    let t46144 = t10199 * t666;
    let t46146 = t2289 * t2341;
    (t46125, t46126, t46129, t46137, t46143, t46144, t46146)
}
