//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1484/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1484(t31439: f64, t8315: f64, t1509: f64, t661: f64, t31149: f64, t2: f64, t31035: f64, t31134: f64, t31135: f64, t31137: f64, t31287: f64, t31415: f64, t31417: f64, t31421: f64, t31424: f64, t31427: f64, t31430: f64, t31434: f64, t31437: f64, t8258: f64, t8267: f64) -> (f64, f64, f64, f64) {
    let t31440 = t8315 * t31439;
    let t31443 = t1509 * t661;
    let t31444 = t31149 * t31443;
    let t31447 = t8315 * t2;
    let t31450 = -t31134 - 2.0_f64 / 3.0_f64 * t31135 + 5.0_f64 / 9.0_f64 * t31137 - 2.0_f64 / 3.0_f64 * t31415 - 3.0_f64 / 4.0_f64 * t31035 * t31417 + 5.0_f64 / 12.0_f64 * t8258 * t31421 + t8258 * t31424 / 4.0_f64 - 5.0_f64 / 9.0_f64 * t31427 - 5.0_f64 / 12.0_f64 * t8258 * t31430 + 25.0_f64 / 72.0_f64 * t8267 * t31434 + 5.0_f64 / 9.0_f64 * t31437 + 5.0_f64 / 12.0_f64 * t8258 * t31440 - 5.0_f64 / 36.0_f64 * t8267 * t31444 + 5.0_f64 / 24.0_f64 * t31287 * t31447;
    (t31440, t31444, t31447, t31450)
}
