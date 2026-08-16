//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 524/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk524(t114: f64, t1916: f64, t1918: f64, t572: f64, t573: f64, t198: f64, t207: f64, t159: f64, t215: f64, t655: f64, t96: f64, t101: f64, t69: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t115 = 1.0_f64 < t114;
    let t1921 = t1916 * t573 + 3.0_f64 * t1918 * t572;
    let t1940 = t198 * t207;
    let t1941 = t215 * t159;
    let t2174 = t655 * t96;
    let t2175 = t2174 * t101;
    let t2178 = piecewise3(t115, 0.0_f64, -t69 * t2175 / 8.0_f64);
    (t1921, t1940, t1941, t2174, t2175, t2178)
}
