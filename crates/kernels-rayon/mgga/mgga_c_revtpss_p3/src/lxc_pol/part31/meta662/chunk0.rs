//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2242/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2242(t18245: f64, t7003: f64, t1518: f64, t4245: f64, t1937: f64, t1501: f64, t4292: f64, t30138: f64, t6993: f64, t29506: f64, t7316: f64, t30112: f64, t7235: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t109149 = 2.0_f64 * t18245 * t7003;
    let t109150 = t4245 * t1518;
    let t109152 = 4.0_f64 * t109150 * t1937;
    let t109153 = t1501 * t4292;
    let t109155 = 4.0_f64 * t109153 * t1937;
    let t109157 = 4.0_f64 * t30138 * t6993;
    let t109158 = t29506 * t7316;
    let t109159 = t7235 * t30112;
    (t109149, t109150, t109152, t109153, t109155, t109157, t109158, t109159)
}
