//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1776/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1776(t25504: f64, t3141: f64, t3148: f64, t7120: f64, t3123: f64, t7121: f64, t365: f64, sigma0: f64) -> (f64, f64, f64, f64, f64) {
    let t25505 = t3141 * t25504;
    let t25508 = t7120 * t3148;
    let t25509 = t3141 * t25508;
    let t25512 = t3123 * t7121;
    let t25515 = sigma0 * t365;
    (t25505, t25508, t25509, t25512, t25515)
}
