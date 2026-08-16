//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2917/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2917(t1432: f64, t4107: f64, t9288: f64, t10107: f64, t3964: f64, t9285: f64, t39494: f64, t4096: f64, t40270: f64, t4089: f64, t138: f64, t2438: f64, t4131: f64, t9674: f64) -> (f64, f64, f64, f64, f64) {
    let t47444 = t1432 * t4107 * t9288;
    let t47450 = t3964 * t10107 * t9285;
    let t47454 = 0.20561456923286030469e-1_f64 * t3964 * t4096 * t39494;
    let t47455 = t40270 * t4089;
    let t47466 = t9674 * t138 * t2438 * t4131;
    (t47444, t47450, t47454, t47455, t47466)
}
