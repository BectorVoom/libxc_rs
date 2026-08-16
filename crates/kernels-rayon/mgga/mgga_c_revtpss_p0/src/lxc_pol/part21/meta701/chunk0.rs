//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2524/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2524(t46089: f64, t10414: f64, t116: f64, t112: f64, t10199: f64, t666: f64, t2289: f64, t2341: f64, t2367: f64, t10210: f64, t625: f64, t10214: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t46090 = 20944.0_f64 / 81.0_f64 * t46089;
    let t46126 = t10414 * t116;
    let t46143 = 2618.0_f64 / 81.0_f64 * t46089 * t112;
    let t46144 = t10199 * t666;
    let t46146 = t2289 * t2341;
    let t46148 = t2289 * t2367;
    let t46150 = t625 * t10210;
    let t46152 = t625 * t10214;
    (t46090, t46126, t46143, t46144, t46146, t46148, t46150, t46152)
}
