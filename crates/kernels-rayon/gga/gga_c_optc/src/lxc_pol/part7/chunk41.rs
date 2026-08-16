//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 41/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk41(t40: f64, t88: f64, t60: f64, t85: f64) -> (f64, f64, f64) {
    let pi = (M_PI as f64);
    let t89 = t40 * t88;
    let t91 = 0.19751789702565206229e-1_f64 * t60 * t85;
    let t92 = pi * pi;
    (t89, t91, t92)
}
