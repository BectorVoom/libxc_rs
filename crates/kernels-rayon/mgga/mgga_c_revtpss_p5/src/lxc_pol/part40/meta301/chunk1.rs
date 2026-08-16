//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1065/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1065(t2778: f64, t9303: f64, t871: f64, t9292: f64, t2760: f64, t72: f64, t686: f64, t874: f64, t251: f64, t9646: f64, t22: f64, t780: f64) -> (f64, f64, f64, f64, f64) {
    let t10969 = 0.26019841438354088051e-2_f64 * t9303 * t2778;
    let t10971 = 0.17073386770573548589e-1_f64 * t9292 * t871;
    let t10972 = t2760 * t72;
    let t10974 = t874 * t10972 * t686;
    let t10981 = t9646 * t251;
    let t10982 = t780 * t22;
    (t10969, t10971, t10974, t10981, t10982)
}
