//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2205/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2205(t1936: f64, t21658: f64, t651: f64, t18245: f64, t7003: f64, t1518: f64, t4245: f64, t1937: f64, t1501: f64, t4292: f64, t30138: f64, t6993: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t109147 = 2.0_f64 * t651 * t21658 * t1936;
    let t109149 = 2.0_f64 * t18245 * t7003;
    let t109150 = t4245 * t1518;
    let t109152 = 4.0_f64 * t109150 * t1937;
    let t109153 = t1501 * t4292;
    let t109155 = 4.0_f64 * t109153 * t1937;
    let t109157 = 4.0_f64 * t30138 * t6993;
    (t109147, t109149, t109150, t109152, t109153, t109155, t109157)
}
