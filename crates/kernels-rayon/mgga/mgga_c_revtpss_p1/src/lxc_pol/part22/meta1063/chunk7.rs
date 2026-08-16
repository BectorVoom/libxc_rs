//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3811/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3811(t46279: f64, t46281: f64, t46286: f64, t46302: f64, t3857: f64, t6801: f64, t14304: f64, t21969: f64, t39419: f64, t39422: f64, t4139: f64, t4140: f64, t46289: f64, t46297: f64, t5541: f64, t5542: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t73314 = 24.0_f64 * t46279;
    let t73315 = 120.0_f64 * t46281;
    let t73316 = 0.11696447245269292414e1_f64 * t46286;
    let t73317 = 24.0_f64 * t46302;
    let t73321 = t3857 * t6801;
    let t73322 = 20.0_f64 * t73321;
    let t73326 = -2.0_f64 * t14304 * t5541 * t5542 + 6.0_f64 * t21969 * t4139 * t4140 - t39419 - t39422 + t46289 - t46297 - t73314 + t73315 - t73316 - t73317 + t73322;
    (t73314, t73315, t73316, t73317, t73322, t73326)
}
