//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 986/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk986(t7984: f64, t8998: f64, t556: f64, t7932: f64, t32003: f64, t4210: f64, t32130: f64, t7965: f64, t2131: f64, t2147: f64, t309: f64, t8392: f64) -> (f64, f64, f64, f64, f64) {
    let t33533 = 0.17347256376410398924e1_f64 * t8998 * t7984;
    let t33535 = t7932 * t556;
    let t33538 = 0.34694512752820797848e1_f64 * t32003 * t33535 * t4210;
    let t33541 = 0.34694512752820797848e1_f64 * t32130 * t33535 * t7965;
    let t33546 = 0.34694512752820797848e1_f64 * t2131 * t2147 * t8392 * t309;
    (t33533, t33535, t33538, t33541, t33546)
}
