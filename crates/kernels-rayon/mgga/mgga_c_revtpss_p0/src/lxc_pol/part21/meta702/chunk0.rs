//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2525/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2525(t10255: f64, t625: f64, t10207: f64, t111: f64, t36227: f64, t36415: f64, t3860: f64, t4029: f64, t3857: f64, t4038: f64, t9387: f64, t2608: f64, t3850: f64, t512: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t46154 = t625 * t10255;
    let t46157 = 1.0_f64 / t10207 / t111;
    let t46196 = 1.0_f64 / t36227;
    let t46212 = 1.0_f64 / t36415;
    let t46279 = t3860 * t4029;
    let t46281 = t3857 * t4029;
    let t46286 = t4038 * t9387;
    let t46289 = t512 * t3850 * t2608;
    (t46154, t46157, t46196, t46212, t46279, t46281, t46286, t46289)
}
