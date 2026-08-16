//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1691/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1691(t10317: f64, t10318: f64, t10326: f64, t10327: f64, t10328: f64, t10331: f64, t1927: f64, t2252: f64, t2258: f64, t2259: f64, t2291: f64, t2312: f64, t36: f64, t38: f64, t39449: f64, t39457: f64, t46091: f64, t606: f64, t627: f64, t641: f64, t6977: f64, t70: f64, t72: f64, t85: f64) -> f64 {
    let t46119 = -t2252 * t2312 / 2.0_f64 + t38 * t46091 * t85 / 24.0_f64 - t39449 * t70 * t85 / 4.0_f64 - t606 * t627 * t72 * t10318 - t10317 * t6977 * t2258 - t10317 * t1927 * t10326 / 3.0_f64 - t36 * t39457 * t70 * t85 / 12.0_f64 - t10327 * t627 * t85 / 3.0_f64 - t10328 * t641 / 3.0_f64 - t2259 * t2291 * t85 / 2.0_f64 - t10331 * t641;
    t46119
}
