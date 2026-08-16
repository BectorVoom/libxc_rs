//! GGA_C_GAPC lxc pol — lxc_pol part 32 (v4rho2sigma2_11) CSE chunk 261/1311 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part32_v4rho2sigma2_11_chunk261(t1006: f64, t1007: f64, t1002: f64, t992: f64, t125: f64, t594: f64, t169: f64) -> (f64, f64, f64) {
    let t1008 = t1006 * t1007;
    let t1010 = 0.10427789137624512459e-2_f64 * t992 + 0.30368356656884499037e-4_f64 * t1002 - 0.21724560703384400956e-4_f64 * t1008;
    let t1012 = t594 * t125;
    let t1013 = t169 * t1012;
    (t1010, t1012, t1013)
}
