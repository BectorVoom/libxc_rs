//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1162/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1162(t13287: f64, t31443: f64, t39858: f64, t2297: f64, t8406: f64, t13299: f64, t31115: f64, t1788: f64, t31110: f64, t2041: f64, t5632: f64, t1805: f64, t7329: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t40114 = t31443 * t13287 * t39858;
    let t40116 = t2297 * t8406;
    let t40118 = t31115 * t13299 * t40116;
    let t40121 = t31110 * t1788;
    let t40123 = t2041 * t5632;
    let t40126 = t7329 * t1805;
    (t40114, t40116, t40118, t40121, t40123, t40126)
}
