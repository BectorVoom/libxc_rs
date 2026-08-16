//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2214/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2214(t2042: f64, t22544: f64, t26123: f64, t572: f64, t5920: f64, t5883: f64, t7002: f64, t101622: f64, t1518: f64, t28276: f64, t4292: f64, t30974: f64, t575: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t109319 = 3.0_f64 * t22544 * t2042;
    let t109322 = 6.0_f64 * t572 * t26123 * t5920;
    let t109327 = 6.0_f64 * t572 * t5883 * t7002;
    let t109330 = 12.0_f64 * t572 * t101622 * t1518;
    let t109333 = 12.0_f64 * t572 * t28276 * t4292;
    let t111419 = t30974 * t575;
    (t109319, t109322, t109327, t109330, t109333, t111419)
}
