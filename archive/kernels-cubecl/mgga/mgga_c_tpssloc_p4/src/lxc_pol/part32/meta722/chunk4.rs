//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2303/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2303<F: Float>(t131: F, t467: F, t5415: F, t6794: F, t29734: F, t607: F, t7376: F, t29754: F, t85853: F, t3032: F, t6224: F, t17691: F, t18301: F, t19173: F, t24589: F, t24788: F, t24812: F, t24849: F, t27549: F, t27550: F, t27551: F, t27638: F, t29749: F, t29776: F, t4978: F, t7373: F, t7375: F, t7378: F, t8066: F, t85859: F, t85963: F, t86015: F, t86037: F, t86076: F, t86077: F, t94948: F, t95000: F, t95005: F, t95035: F) -> (F, F) {
    let t103581 = t5415 * t6794 * t131 * t467;
    let t103593 = t29734 * t7376 * t607;
    let t103610 = t85853 * t29754;
    let t103615 = t6224 * t3032;
    let t103624 = F::cast_from(0.80418998823691070228e-1_f64) * t103581 * t7378 + F::cast_from(0.82246703342411321825e-2_f64) * t7373 * t7375 * t19173 * t7376 - F::cast_from(0.10966227112321509577e-1_f64) * t86037 * t94948 * t29734 * t27638 - F::cast_from(0.10966227112321509577e-1_f64) * t24849 * t86015 * t103593 + F::cast_from(0.73108180748810063846e-2_f64) * t27549 * t27550 * t27551 * t17691 - F::cast_from(0.36554090374405031923e-2_f64) * t27549 * t24788 * t29776 + F::cast_from(0.54831135561607547884e-2_f64) * t24589 * t95035 * t8066 + F::cast_from(0.73108180748810063845e-2_f64) * t86076 * t86077 * t103593 - F::cast_from(0.27415567780803773942e-2_f64) * t103610 - F::cast_from(0.16449340668482264365e-1_f64) * t24812 * t85859 * t29749 + F::cast_from(0.49348022005446793095e-1_f64) * t85963 * t95000 * t103615 * t18301 - F::cast_from(0.49348022005446793095e-1_f64) * t85963 * t95005 * t103615 * t4978;
    (t103615, t103624)
}
