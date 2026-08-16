//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1496/1505 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1496(t31027: f64, t31633: f64, t31640: f64, t625: f64, t105872: f64, t116919: f64, t117183: f64, t117184: f64, t117186: f64, t117976: f64, t117978: f64, t118009: f64, t118011: f64, t31035: f64, t31149: f64, t5891: f64, t5911: f64, t661: f64, t8267: f64, t8311: f64, t8315: f64) -> f64 {
    let t118733 = t31027 * t31633;
    let t118744 = t625 * t31640;
    let t118746 = -5.0_f64 / 36.0_f64 * t8267 * t31149 * t5911 * t661 - t117976 + t117978 - 20.0_f64 / 9.0_f64 * t118733 + 3.0_f64 * t116919 * t8311 * t105872 - 5.0_f64 / 4.0_f64 * t31035 * t8315 * t5891 * t661 + 22.0_f64 / 9.0_f64 * t117184 - 55.0_f64 / 27.0_f64 * t117186 + t117183 + t118009 - t118011 + 40.0_f64 / 27.0_f64 * t118744;
    t118746
}
