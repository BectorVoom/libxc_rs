//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2458/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2458(t39494: f64, t3964: f64, t4096: f64, t40270: f64, t4089: f64, t3911: f64, t9692: f64, t123: f64, t1444: f64, t3915: f64, t9291: f64, t2453: f64, t9679: f64) -> (f64, f64, f64, f64, f64) {
    let t47454 = 0.20561456923286030469e-1_f64 * t3964 * t4096 * t39494;
    let t47455 = t40270 * t4089;
    let t47474 = t3911 * t9692;
    let t47478 = t3915 * t123 * t9291 * t1444;
    let t47480 = t2453 * t9679;
    (t47454, t47455, t47474, t47478, t47480)
}
