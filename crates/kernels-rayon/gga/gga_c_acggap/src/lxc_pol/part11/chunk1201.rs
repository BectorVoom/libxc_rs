//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1201/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1201(t309: f64, t406: f64, t944: f64, t33673: f64, t7963: f64, t524: f64, t7932: f64, t4210: f64, t7942: f64, t7965: f64, t2131: f64, t2132: f64, t8993: f64) -> (f64, f64, f64, f64, f64) {
    let t36429 = t944 * t309 * t406;
    let t36432 = 0.34694512752820797848e1_f64 * t7963 * t33673 * t36429;
    let t36433 = t7932 * t524;
    let t36436 = 0.17347256376410398924e1_f64 * t7942 * t36433 * t4210;
    let t36439 = 0.17347256376410398924e1_f64 * t7963 * t36433 * t7965;
    let t36447 = 0.17347256376410398924e1_f64 * t2131 * t2132 * t8993 * t309;
    (t36432, t36433, t36436, t36439, t36447)
}
