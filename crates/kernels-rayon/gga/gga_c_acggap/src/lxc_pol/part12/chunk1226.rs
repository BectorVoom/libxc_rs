//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1226/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1226(t2400: f64, t30005: f64, t880: f64, t9380: f64, t2138: f64, t2147: f64, t322: f64, t9413: f64, t524: f64, t8306: f64) -> (f64, f64, f64, f64) {
    let t38073 = t30005 * t2400;
    let t38077 = t9380 * t880;
    let t38085 = 0.34694512752820797848e1_f64 * t2138 * t2147 * t9413 * t322;
    let t38086 = t8306 * t524;
    (t38073, t38077, t38085, t38086)
}
