//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2272/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2272(t109291: f64, t109293: f64, t109295: f64, t109299: f64, t109305: f64, t109307: f64, t109310: f64, t109315: f64, t109319: f64, t109322: f64, t109327: f64, t109330: f64, t109333: f64, t113015: f64, t1461: f64, t1918: f64, t29480: f64, t30985: f64, t573: f64, t5805: f64, t6948: f64, t7696: f64, t8245: f64, param_d: f64) -> f64 {
    let t113050 = t113015 * t573 * param_d + 3.0_f64 * t1461 * t30985 + 6.0_f64 * t1918 * t29480 + 6.0_f64 * t5805 * t8245 + 3.0_f64 * t6948 * t7696 + t109291 + t109293 + t109295 + t109299 + t109305 + t109307 + t109310 + t109315 + t109319 + t109322 + t109327 + t109330 + t109333;
    t113050
}
