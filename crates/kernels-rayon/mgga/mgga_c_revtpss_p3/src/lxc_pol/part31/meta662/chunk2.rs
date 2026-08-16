//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2244/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2244(t28187: f64, t7898: f64, t30110: f64, t531: f64, t2014: f64, t7238: f64, t28043: f64, t7732: f64, t28021: f64, t28173: f64, t1937: f64, t75439: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t109172 = 2.0_f64 * t7898 * t28187;
    let t109173 = t531 * t30110;
    let t109176 = 3.0_f64 * t2014 * t109173 * t7238;
    let t109178 = 4.0_f64 * t7732 * t28043;
    let t109180 = 2.0_f64 * t7898 * t28021;
    let t109182 = 6.0_f64 * t7898 * t28173;
    let t109194 = 2.0_f64 * t75439 * t1937;
    (t109172, t109176, t109178, t109180, t109182, t109194)
}
