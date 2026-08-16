//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1583/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1583(t15125: f64, t15191: f64, t11133: f64, t11134: f64, t11136: f64, t11138: f64, t11140: f64, t15127: f64, t15132: f64, t15137: f64, t15142: f64, t15147: f64, t15151: f64, t15156: f64, t15160: f64, t15189: f64, t15195: f64) -> f64 {
    let t15638 = 0.19755555555555555556e-1_f64 * t15125;
    let t15639 = 0.9877777777777777778e-2_f64 * t15191;
    let t15648 = -t11133 - 0.13170370370370370371e-1_f64 * t11134 + 0.32925925925925925927e-2_f64 * t11136 - 0.9877777777777777778e-2_f64 * t11138 + 0.4938888888888888889e-2_f64 * t11140 - 0.65851851851851851853e-2_f64 * t15189 + 0.65851851851851851854e-2_f64 * t15127 - t15638 + t15639 - 0.16462962962962962963e-1_f64 * t15142 + 0.59266666666666666668e-1_f64 * t15156 - 0.19755555555555555556e-1_f64 * t15132 - 0.9877777777777777778e-2_f64 * t15137 - 0.88900000000000000002e-1_f64 * t15160 + 0.59266666666666666668e-1_f64 * t15147 + 0.29633333333333333334e-1_f64 * t15151 - 0.14816666666666666667e-1_f64 * t15195;
    t15648
}
