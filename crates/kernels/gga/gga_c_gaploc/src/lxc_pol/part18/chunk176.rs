//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 176/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk176<F: Float>(t475: F, t600: F, t568: F, t190: F, t193: F, t199: F, t205: F, t525: F, t530: F, t532: F, t536: F, t541: F, t547: F, t552: F, t557: F, t558: F, t562: F, t567: F, t571: F, t574: F, t576: F, t581: F, t587: F, t591: F, t597: F) -> (F, F, F) {
    let t601 = t600 * t475;
    let t602 = t568 * t601;
    let t605 = 0.35750489951850426669e0 * t525 * t193 - 0.35750489951850426669e0 * t530 * t532 + 0.35750489951850426669e0 * t536 * t193 + 0.23833659967900284446e0 * t190 * t541 - 0.39722766613167140743e-1 * t547 * t552 - 0.35750489951850426669e0 * t557 * t558 - 0.11502877786176224903e1 * t562 * t205 + 0.11502877786176224903e1 * t567 * t571 - 0.23005755572352449806e1 * t574 * t576 - 0.15337170381568299871e1 * t199 * t581 + 0.25561950635947166451e0 * t587 * t591 + 0.23005755572352449806e1 * t597 * t602;
    (t601, t602, t605)
}
