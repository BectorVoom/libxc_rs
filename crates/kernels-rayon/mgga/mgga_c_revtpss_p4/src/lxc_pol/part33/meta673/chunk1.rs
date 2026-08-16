//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2204/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2204(t2014: f64, t22483: f64, t7312: f64, t28172: f64, t28176: f64, t29498: f64, t94345: f64, t29583: f64, t7235: f64, t2322: f64, t30128: f64, t4254: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t109128 = t2014 * t7312 * t22483;
    let t109135 = 6.0_f64 * t2014 * t28172 * t28176;
    let t109138 = 6.0_f64 * t2014 * t94345 * t29498;
    let t109140 = 6.0_f64 * t7235 * t29583;
    let t109142 = 2.0_f64 * t2322 * t30128;
    let t109144 = 2.0_f64 * t4254 * t30128;
    (t109128, t109135, t109138, t109140, t109142, t109144)
}
