//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2842/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2842(t2857: f64, t3154: f64, t2251: f64, t11262: f64, t3127: f64, t3129: f64, t11988: f64, t3106: f64, t271: f64, t2852: f64, t1054: f64, t11970: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t43174 = t3154 * t2857;
    let t43175 = t43174 * t2251;
    let t43204 = t3127 * t11262 * t3129;
    let t43215 = t3106 * t11988;
    let t43222 = 1.0_f64 / t271 / t2852;
    let t43238 = t1054 * t11970;
    (t43174, t43175, t43204, t43215, t43222, t43238)
}
