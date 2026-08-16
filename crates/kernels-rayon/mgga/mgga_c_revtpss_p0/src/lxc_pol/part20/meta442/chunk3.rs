//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1690/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1690(t46: f64, t47: f64, t58: f64, t59: f64, t2681: f64, t64: f64, t10326: f64, t10345: f64, t10355: f64, t10357: f64, t10360: f64, t10361: f64, t10364: f64, t10368: f64, t10372: f64, t2251: f64, t2258: f64, t2270: f64, t2275: f64, t2276: f64, t2279: f64, t2282: f64, t39443: f64, t39449: f64, t39457: f64, t42748: f64, t44: f64, t48: f64, t49: f64, t56: f64, t60: f64, t614: f64, t617: f64) -> (f64, f64) {
    let t46063 = t46 * t46;
    let t46065 = 1.0_f64 / t47 / t46063;
    let t46072 = t58 * t58;
    let t46074 = 1.0_f64 / t59 / t46072;
    let t46089 = t64 * t2681;
    let t46090 = 20944.0_f64 / 81.0_f64 * t46089;
    let t46091 = 10.0_f64 / 9.0_f64 * t56 * t10372 * t10326 - 80.0_f64 / 9.0_f64 * t614 * t10361 - 5.0_f64 / 18.0_f64 * t44 * t10355 * t2251 * t2258 + 5.0_f64 / 6.0_f64 * t44 * t2275 * t39449 + 10.0_f64 / 9.0_f64 * t44 * t10360 * t10326 + 5.0_f64 / 18.0_f64 * t56 * t10368 * t2251 * t2258 + 5.0_f64 / 6.0_f64 * t56 * t2282 * t39449 + 40.0_f64 / 81.0_f64 * t614 * t10357 - 80.0_f64 / 9.0_f64 * t614 * t10364 + 5.0_f64 / 162.0_f64 * t44 * t46065 * t39443 + 5.0_f64 / 6.0_f64 * t44 * t48 * t39457 + 5.0_f64 / 162.0_f64 * t56 * t46074 * t39443 - 5.0_f64 / 6.0_f64 * t56 * t60 * t39457 + 20944.0_f64 / 81.0_f64 * t42748 * t49 - 12320.0_f64 / 81.0_f64 * t10345 * t617 + 440.0_f64 / 9.0_f64 * t2270 * t2279 + 440.0_f64 / 27.0_f64 * t2270 * t2276 - t46090;
    (t46089, t46091)
}
