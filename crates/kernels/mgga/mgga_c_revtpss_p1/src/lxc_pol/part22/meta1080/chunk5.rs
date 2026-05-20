//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3887/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3887<F: Float>(t22026: F, t46802: F, t9794: F, t46694: F, t6850: F, t13783: F, t13867: F, t13872: F, t1883: F, t221: F, t3934: F, t47320: F, t49093: F, t49105: F, t49118: F, t49121: F, t49124: F, t5591: F, t5627: F, t5659: F) -> F {
    let t74677 = t46802 * t9794 * t22026;
    let t74682 = t46694 * t6850;
    let t74696 = F::cast_from(0.90357964994909313586e-6_f64) * t49105 + F::cast_from(0.60976381323476959248e-3_f64) * t47320 - t49093 * t221 * t5627 * t5591 + F::cast_from(0.90357964994909313586e-5_f64) * t74677 + F::cast_from(0.40015750243531754508e-2_f64) * t49118 - F::cast_from(0.4065600224742826258e-4_f64) * t49121 - F::cast_from(0.18071592998981862716e-4_f64) * t49124 + F::new(35.0) / F::new(72.0) * t74682 - F::cast_from(0.17149607247227894789e-1_f64) * t3934 * t13783 * t5659 * t5627 - F::cast_from(0.17149607247227894789e-1_f64) * t3934 * t13783 * t1883 * t13867 - F::cast_from(0.85748036236139473945e-2_f64) * t3934 * t13783 * t1883 * t13872;
    t74696
}
