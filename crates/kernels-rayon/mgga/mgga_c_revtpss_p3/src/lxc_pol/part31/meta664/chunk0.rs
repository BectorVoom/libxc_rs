//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2254/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2254(t28264: f64, t572: f64, t5920: f64, t105886: f64, t117: f64, t2042: f64, t22544: f64, t26123: f64, t5883: f64, t7002: f64, t101622: f64, t1518: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t109310 = 6.0_f64 * t572 * t28264 * t5920;
    let t109315 = 3.0_f64 * t572 * t117 * t105886;
    let t109319 = 3.0_f64 * t22544 * t2042;
    let t109322 = 6.0_f64 * t572 * t26123 * t5920;
    let t109327 = 6.0_f64 * t572 * t5883 * t7002;
    let t109330 = 12.0_f64 * t572 * t101622 * t1518;
    (t109310, t109315, t109319, t109322, t109327, t109330)
}
