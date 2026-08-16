//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 943/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk943(t1904: f64, t5599: f64, t689: f64, t212: f64, t6888: f64, t1358: f64, t1357: f64, t6896: f64, t6895: f64, t72: f64, t686: f64, t9680: f64) -> (f64, f64, f64, f64, f64) {
    let t22427 = t5599 * t1904;
    let t22428 = t689 * t22427;
    let t22445 = t212 * t6888;
    let t22446 = t22445 * t1358;
    let t22447 = t689 * t22446;
    let t22449 = t1357 * t6896;
    let t22450 = t689 * t22449;
    let t22452 = t6895 * t72;
    let t22453 = t22452 * t686;
    let t22454 = t9680 * t22453;
    (t22428, t22447, t22450, t22453, t22454)
}
