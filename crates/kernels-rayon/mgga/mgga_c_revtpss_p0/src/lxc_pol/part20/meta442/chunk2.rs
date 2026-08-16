//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1689/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1689(t12267: f64, t81: f64, t10321: f64, t10326: f64, t10336: f64, t10380: f64, t10381: f64, t10389: f64, t10392: f64, t10398: f64, t10401: f64, t10407: f64, t2251: f64, t2258: f64, t2260: f64, t2263: f64, t2291: f64, t2292: f64, t2299: f64, t2306: f64, t2312: f64, t39443: f64, t39449: f64, t39457: f64, t46001: f64, t607: f64, t608: f64, t628: f64, t633: f64, t637: f64, t641: f64, t71: f64, t77: f64, t85: f64) -> f64 {
    let t46014 = 1.0_f64 / t81 / t12267;
    let t46034 = -t2260 * t2312 / 2.0_f64 - t607 * t10380 * t85 / 3.0_f64 - t10336 * t641 - t2263 * t2312 - t608 * t10407 / 3.0_f64 + t10381 * t641 / 6.0_f64 + t2292 * t2312 / 4.0_f64 + t628 * t10407 / 6.0_f64 + t71 * t77 * (3640.0_f64 / 81.0_f64 * t46001 * t39443 - 560.0_f64 / 9.0_f64 * t10389 * t2251 * t2258 + 28.0_f64 / 3.0_f64 * t2299 * t39449 + 112.0_f64 / 9.0_f64 * t10392 * t10326 - 4.0_f64 / 3.0_f64 * t633 * t39457 + 3640.0_f64 / 81.0_f64 * t46014 * t39443 + 560.0_f64 / 9.0_f64 * t10398 * t2251 * t2258 + 28.0_f64 / 3.0_f64 * t2306 * t39449 + 112.0_f64 / 9.0_f64 * t10401 * t10326 + 4.0_f64 / 3.0_f64 * t637 * t39457) / 24.0_f64 - t2251 * t2291 * t85 / 2.0_f64 - t10321 * t641;
    t46034
}
