//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2255/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2255(t28276: f64, t4292: f64, t572: f64, t109291: f64, t109293: f64, t109295: f64, t109299: f64, t109305: f64, t109307: f64, t109310: f64, t109315: f64, t109319: f64, t109322: f64, t109327: f64, t109330: f64, t1918: f64, t2040: f64, t22559: f64, t22565: f64, t28246: f64, t5802: f64, t6948: f64, t7324: f64, t7944: f64) -> f64 {
    let t109333 = 12.0_f64 * t572 * t28276 * t4292;
    let t109334 = 6.0_f64 * t1918 * t28246 + 12.0_f64 * t2040 * t22559 + 6.0_f64 * t2040 * t22565 + 12.0_f64 * t5802 * t7944 + 3.0_f64 * t6948 * t7324 + t109291 + t109293 + t109295 + t109299 + t109305 + t109307 + t109310 + t109315 + t109319 + t109322 + t109327 + t109330 + t109333;
    t109334
}
