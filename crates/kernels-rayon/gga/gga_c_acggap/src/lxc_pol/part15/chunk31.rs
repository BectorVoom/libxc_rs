//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 31/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk31(t40: f64, t88: f64, t60: f64, t85: f64) -> (f64, f64, f64, f64) {
    let pi = (M_PI as f64);
    let t89 = t40 * t88;
    let t91 = 0.19751673498613801407e-1_f64 * t60 * t85;
    let t92 = f64::ln(2.0_f64);
    let t93 = 1.0_f64 - t92;
    let t94 = pi * pi;
    (t89, t91, t93, t94)
}
