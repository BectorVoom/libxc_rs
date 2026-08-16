//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1290/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1290(t15474: f64, t935: f64, t915: f64, t15127: f64, t15125: f64, t15191: f64, t11134: f64, t11136: f64, t11138: f64, t11140: f64, t11560: f64, t15132: f64, t15137: f64, t15142: f64, t15147: f64, t15151: f64, t15156: f64, t15160: f64, t15189: f64, t15195: f64) -> (f64, f64) {
    let t15475 = t15474 * t935;
    let t15477 = 1.0_f64 * t915 * t15475;
    let t15483 = 0.41203703703703703704e-2_f64 * t15127;
    let t15484 = 0.12361111111111111111e-1_f64 * t15125;
    let t15485 = 0.61805555555555555556e-2_f64 * t15191;
    let t15494 = -t11560 - 0.82407407407407407407e-2_f64 * t11134 + 0.20601851851851851852e-2_f64 * t11136 - 0.61805555555555555556e-2_f64 * t11138 + 0.30902777777777777778e-2_f64 * t11140 - 0.41203703703703703704e-2_f64 * t15189 + t15483 - t15484 + t15485 - 0.10300925925925925926e-1_f64 * t15142 + 0.37083333333333333333e-1_f64 * t15156 - 0.12361111111111111111e-1_f64 * t15132 - 0.61805555555555555555e-2_f64 * t15137 - 0.55625000000000000001e-1_f64 * t15160 + 0.37083333333333333334e-1_f64 * t15147 + 0.18541666666666666667e-1_f64 * t15151 - 0.92708333333333333333e-2_f64 * t15195;
    (t15477, t15494)
}
