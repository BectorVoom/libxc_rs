//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 138/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk138(t475: f64, t600: f64, t568: f64, t190: f64, t193: f64, t199: f64, t205: f64, t525: f64, t530: f64, t532: f64, t536: f64, t541: f64, t547: f64, t552: f64, t557: f64, t558: f64, t562: f64, t567: f64, t571: f64, t574: f64, t576: f64, t581: f64, t587: f64, t591: f64, t597: f64) -> f64 {
    let t601 = t600 * t475;
    let t602 = t568 * t601;
    let t605 = 0.35750489951850426669e0_f64 * t525 * t193 - 0.35750489951850426669e0_f64 * t530 * t532 + 0.35750489951850426669e0_f64 * t536 * t193 + 0.23833659967900284446e0_f64 * t190 * t541 - 0.39722766613167140743e-1_f64 * t547 * t552 - 0.35750489951850426669e0_f64 * t557 * t558 - 0.11502877786176224903e1_f64 * t562 * t205 + 0.11502877786176224903e1_f64 * t567 * t571 - 0.23005755572352449806e1_f64 * t574 * t576 - 0.15337170381568299871e1_f64 * t199 * t581 + 0.25561950635947166451e0_f64 * t587 * t591 + 0.23005755572352449806e1_f64 * t597 * t602;
    t605
}
