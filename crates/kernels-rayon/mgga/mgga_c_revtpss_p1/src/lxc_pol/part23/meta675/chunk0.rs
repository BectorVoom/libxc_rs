//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2412/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2412(t43813: f64, t241: f64, t281: f64, t414: f64, t39484: f64, t403: f64, t409: f64, t13099: f64, t159: f64, t1123: f64, t9292: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t43814 = 0.31310740740740740741e1_f64 * t43813;
    let t43816 = t281 * t241 * t414;
    let t43817 = 0.13490888888888888889e1_f64 * t43816;
    let t43821 = 1.0_f64 / t409 / t39484 / t403 / 96.0_f64;
    let t43860 = t159 * t13099;
    let t43881 = 280.0_f64 / 81.0_f64 * t43813;
    let t43888 = t9292 * t1123;
    (t43814, t43816, t43817, t43821, t43860, t43881, t43888)
}
