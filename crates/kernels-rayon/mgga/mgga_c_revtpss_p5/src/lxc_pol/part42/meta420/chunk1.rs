//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1481/1505 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1481(t5911: f64, t8315: f64, t31035: f64, t31134: f64, t31415: f64, t31427: f64, t31437: f64, t31626: f64, t31629: f64, t31633: f64, t31636: f64, t31640: f64, t31643: f64, t31646: f64, t69: f64, t8258: f64, t8267: f64) -> (f64, f64) {
    let t31649 = t8315 * t5911;
    let t31652 = -t31134 - 4.0_f64 / 3.0_f64 * t31415 - 10.0_f64 / 9.0_f64 * t31427 + 10.0_f64 / 9.0_f64 * t31437 - 3.0_f64 / 4.0_f64 * t31035 * t31626 - 5.0_f64 / 6.0_f64 * t8258 * t31629 + 5.0_f64 / 6.0_f64 * t8258 * t31633 + t8258 * t31636 / 4.0_f64 - 5.0_f64 / 9.0_f64 * t69 * t31640 + 25.0_f64 / 36.0_f64 * t8267 * t31643 - 5.0_f64 / 36.0_f64 * t8267 * t31646 - 5.0_f64 / 24.0_f64 * t8267 * t31649;
    (t31649, t31652)
}
