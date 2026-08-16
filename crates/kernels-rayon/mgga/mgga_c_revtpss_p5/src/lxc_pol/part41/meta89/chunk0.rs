//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 517/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk517(t1916: f64, t1918: f64, t572: f64, t573: f64, t76: f64, t84: f64, t198: f64, t207: f64, t159: f64, t215: f64, t655: f64, t96: f64) -> (f64, f64, f64, f64, f64) {
    let t1921 = t1916 * t573 + 3.0_f64 * t1918 * t572;
    let t1927 = t76 * t84;
    let t1940 = t198 * t207;
    let t1941 = t215 * t159;
    let t2174 = t655 * t96;
    (t1921, t1927, t1940, t1941, t2174)
}
