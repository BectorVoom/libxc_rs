//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2206/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2206(t29506: f64, t7316: f64, t30112: f64, t7235: f64, t27833: f64, t7937: f64, t28189: f64, t7898: f64, t7239: f64, t2014: f64, t30111: f64, t7315: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t109158 = t29506 * t7316;
    let t109159 = t7235 * t30112;
    let t109162 = 2.0_f64 * t27833 * t7937;
    let t109164 = 2.0_f64 * t7898 * t28189;
    let t109167 = 3.0_f64 * t29506 * t7239;
    let t109169 = t2014 * t30111 * t7315;
    (t109158, t109159, t109162, t109164, t109167, t109169)
}
