//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 138/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk138<F: Float>(t475: F, t600: F, t568: F, t190: F, t193: F, t199: F, t205: F, t525: F, t530: F, t532: F, t536: F, t541: F, t547: F, t552: F, t557: F, t558: F, t562: F, t567: F, t571: F, t574: F, t576: F, t581: F, t587: F, t591: F, t597: F) -> F {
    let t601 = t600 * t475;
    let t602 = t568 * t601;
    let t605 = F::cast_from(0.35750489951850426669e0_f64) * t525 * t193 - F::cast_from(0.35750489951850426669e0_f64) * t530 * t532 + F::cast_from(0.35750489951850426669e0_f64) * t536 * t193 + F::cast_from(0.23833659967900284446e0_f64) * t190 * t541 - F::cast_from(0.39722766613167140743e-1_f64) * t547 * t552 - F::cast_from(0.35750489951850426669e0_f64) * t557 * t558 - F::cast_from(0.11502877786176224903e1_f64) * t562 * t205 + F::cast_from(0.11502877786176224903e1_f64) * t567 * t571 - F::cast_from(0.23005755572352449806e1_f64) * t574 * t576 - F::cast_from(0.15337170381568299871e1_f64) * t199 * t581 + F::cast_from(0.25561950635947166451e0_f64) * t587 * t591 + F::cast_from(0.23005755572352449806e1_f64) * t597 * t602;
    t605
}
