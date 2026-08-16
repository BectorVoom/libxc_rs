//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 847/1426 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk847(t5542: f64, t7953: f64, t291: f64, t5211: f64, t7956: f64, t3127: f64, t3402: f64, t7944: f64, t3132: f64, t7259: f64, t2492: f64, t2701: f64, t646: f64) -> (f64, f64, f64, f64) {
    let t9677 = t7953 * t5542;
    let t9679 = t5211 * t291 * t7956;
    let t9680 = t9677 * t9679;
    let t9682 = t3402 * t3127;
    let t9683 = t9682 * t7944;
    let t9685 = t7259 * t3132;
    let t9686 = t9685 * t7944;
    let t9689 = t646 * t2492 * t2701;
    (t9680, t9683, t9686, t9689)
}
