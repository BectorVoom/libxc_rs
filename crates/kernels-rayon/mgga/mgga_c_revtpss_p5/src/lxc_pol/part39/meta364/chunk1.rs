//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1272/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1272(t4628: f64, t698: f64, t15193: f64, t930: f64, t141: f64, t15127: f64, t15125: f64, t15191: f64, t11134: f64, t11136: f64, t11138: f64, t11140: f64, t11304: f64, t15132: f64, t15137: f64, t15142: f64, t15147: f64, t15151: f64, t15156: f64, t15160: f64, t15189: f64, t15195: f64) -> (f64, f64, f64, f64) {
    let t15197 = t698 * t4628;
    let t15198 = 0.11038e0_f64 * t15197;
    let t15199 = t930 * t15193;
    let t15200 = t141 * t15199;
    let t15209 = 4.0_f64 / 27.0_f64 * t15127;
    let t15210 = 4.0_f64 / 9.0_f64 * t15125;
    let t15211 = 2.0_f64 / 9.0_f64 * t15191;
    let t15220 = -t11304 - 8.0_f64 / 27.0_f64 * t11134 + 2.0_f64 / 27.0_f64 * t11136 - 2.0_f64 / 9.0_f64 * t11138 + t11140 / 9.0_f64 - 4.0_f64 / 27.0_f64 * t15189 + t15209 - t15210 + t15211 - 10.0_f64 / 27.0_f64 * t15142 + 4.0_f64 / 3.0_f64 * t15156 - 4.0_f64 / 9.0_f64 * t15132 - 2.0_f64 / 9.0_f64 * t15137 - 2.0_f64 * t15160 + 4.0_f64 / 3.0_f64 * t15147 + 2.0_f64 / 3.0_f64 * t15151 - t15195 / 3.0_f64;
    (t15197, t15198, t15200, t15220)
}
