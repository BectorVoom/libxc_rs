//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2477/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2477(t43813: f64, t241: f64, t281: f64, t414: f64, t39484: f64, t403: f64, t409: f64, t12288: f64, t698: f64, t12316: f64, t689: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t43814 = 0.31310740740740740741e1_f64 * t43813;
    let t43816 = t281 * t241 * t414;
    let t43817 = 0.13490888888888888889e1_f64 * t43816;
    let t43821 = 1.0_f64 / t409 / t39484 / t403 / 96.0_f64;
    let t43828 = t698 * t12288;
    let t43830 = t689 * t12316;
    (t43814, t43816, t43817, t43821, t43828, t43830)
}
