//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3887/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3887(t22026: f64, t46802: f64, t9794: f64, t46694: f64, t6850: f64, t13783: f64, t13867: f64, t13872: f64, t1883: f64, t221: f64, t3934: f64, t47320: f64, t49093: f64, t49105: f64, t49118: f64, t49121: f64, t49124: f64, t5591: f64, t5627: f64, t5659: f64) -> f64 {
    let t74677 = t46802 * t9794 * t22026;
    let t74682 = t46694 * t6850;
    let t74696 = 0.90357964994909313586e-6_f64 * t49105 + 0.60976381323476959248e-3_f64 * t47320 - t49093 * t221 * t5627 * t5591 + 0.90357964994909313586e-5_f64 * t74677 + 0.40015750243531754508e-2_f64 * t49118 - 0.4065600224742826258e-4_f64 * t49121 - 0.18071592998981862716e-4_f64 * t49124 + 35.0_f64 / 72.0_f64 * t74682 - 0.17149607247227894789e-1_f64 * t3934 * t13783 * t5659 * t5627 - 0.17149607247227894789e-1_f64 * t3934 * t13783 * t1883 * t13867 - 0.85748036236139473945e-2_f64 * t3934 * t13783 * t1883 * t13872;
    t74696
}
