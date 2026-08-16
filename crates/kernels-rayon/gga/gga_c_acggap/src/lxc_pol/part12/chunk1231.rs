//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1231/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1231(t29979: f64, t36417: f64, t638: f64, t2132: f64, t322: f64, t7896: f64, t9431: f64, t119: f64, t9367: f64, t2395: f64, t30005: f64, t8081: f64, t8998: f64) -> (f64, f64, f64, f64, f64) {
    let t38181 = t29979 * t638 * t36417;
    let t38185 = t7896 * t2132 * t9431 * t322;
    let t38187 = t119 * t9367;
    let t38190 = t30005 * t2395;
    let t38194 = 0.34694512752820797848e1_f64 * t8998 * t8081;
    (t38181, t38185, t38187, t38190, t38194)
}
