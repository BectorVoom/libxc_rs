//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1435/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1435(t17649: f64, t17650: f64, t17350: f64, t3767: f64, t1121: f64, t1248: f64, t606: f64, t3604: f64, t17353: f64, t372: f64, t5277: f64, t3630: f64) -> (f64, f64, f64, f64) {
    let t17651 = t17649 * t17650;
    let t17654 = t3767 * t17350;
    let t17655 = t1248 * t1121;
    let t17656 = t17655 * t606;
    let t17657 = t3604 * t17656;
    let t17658 = t17353 * t17657;
    let t17661 = t372 * t5277;
    let t17662 = t17661 * t3630;
    (t17651, t17654, t17658, t17662)
}
