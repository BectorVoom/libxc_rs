//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3927/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3927(t1501: f64, t2371: f64, t4292: f64, t21830: f64, t625: f64, t13509: f64, t21820: f64, t21876: f64, t2339: f64, t2340: f64, t2366: f64, t4263: f64, t46143: f64, t46157: f64, t49698: f64, t49700: f64, t49702: f64, t49704: f64, t49724: f64, t49817: f64, t49819: f64, t5891: f64, t665: f64, t69: f64) -> (f64, f64, f64) {
    let t75485 = t1501 * t2371;
    let t75494 = t4292 * t4292;
    let t75526 = t625 * t21830;
    let t75532 = 88.0_f64 / 9.0_f64 * t49700 - 8.0_f64 / 3.0_f64 * t49702 - 4.0_f64 / 3.0_f64 * t49704 + 4.0_f64 * t49819 + t46143 + 2.0_f64 / 3.0_f64 * t49724 - 44.0_f64 / 9.0_f64 * t49817 - 3.0_f64 / 4.0_f64 * t69 * t21820 * t2366 + t69 * t4263 * t13509 / 2.0_f64 + 308.0_f64 / 27.0_f64 * t49698 + 3.0_f64 * t69 * t46157 * t5891 * t2340 - 4.0_f64 / 3.0_f64 * t75526 + t69 * t2339 * t21876 * t665 / 2.0_f64;
    (t75485, t75494, t75532)
}
