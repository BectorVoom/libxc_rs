//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2199/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2199(t18227: f64, t7742: f64, t28063: f64, t4248: f64, t28182: f64, t7898: f64, t29499: f64, t7235: f64, t2014: f64, t29498: f64, t32737: f64, t27137: f64, t7732: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t109043 = 4.0_f64 * t18227 * t7742;
    let t109045 = 4.0_f64 * t4248 * t28063;
    let t109047 = 2.0_f64 * t7898 * t28182;
    let t109049 = 6.0_f64 * t7235 * t29499;
    let t109052 = 6.0_f64 * t2014 * t32737 * t29498;
    let t109054 = 4.0_f64 * t7732 * t27137;
    (t109043, t109045, t109047, t109049, t109052, t109054)
}
