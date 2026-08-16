//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1066/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1066(t32003: f64, t38052: f64, t4210: f64, t8065: f64, t8998: f64, t2400: f64, t30005: f64, t880: f64, t9380: f64, t2138: f64, t2147: f64, t322: f64, t9413: f64) -> (f64, f64, f64, f64, f64) {
    let t38055 = 0.34694512752820797848e1_f64 * t32003 * t38052 * t4210;
    let t38065 = t8998 * t8065;
    let t38073 = t30005 * t2400;
    let t38077 = t9380 * t880;
    let t38085 = 0.34694512752820797848e1_f64 * t2138 * t2147 * t9413 * t322;
    (t38055, t38065, t38073, t38077, t38085)
}
