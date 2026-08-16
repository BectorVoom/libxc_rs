//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2431/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2431(t46089: f64, t112: f64, t10199: f64, t666: f64, t10207: f64, t111: f64, t36227: f64, t36415: f64, t3860: f64, t4029: f64, t3857: f64, t4038: f64, t9387: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t46090 = 20944.0_f64 / 81.0_f64 * t46089;
    let t46143 = 2618.0_f64 / 81.0_f64 * t46089 * t112;
    let t46144 = t10199 * t666;
    let t46157 = 1.0_f64 / t10207 / t111;
    let t46196 = 1.0_f64 / t36227;
    let t46212 = 1.0_f64 / t36415;
    let t46279 = t3860 * t4029;
    let t46281 = t3857 * t4029;
    let t46286 = t4038 * t9387;
    (t46090, t46143, t46144, t46157, t46196, t46212, t46279, t46281, t46286)
}
