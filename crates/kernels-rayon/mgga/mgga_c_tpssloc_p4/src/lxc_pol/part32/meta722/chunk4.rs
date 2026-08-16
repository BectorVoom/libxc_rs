//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2303/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2303(t131: f64, t467: f64, t5415: f64, t6794: f64, t29734: f64, t607: f64, t7376: f64, t29754: f64, t85853: f64, t3032: f64, t6224: f64, t17691: f64, t18301: f64, t19173: f64, t24589: f64, t24788: f64, t24812: f64, t24849: f64, t27549: f64, t27550: f64, t27551: f64, t27638: f64, t29749: f64, t29776: f64, t4978: f64, t7373: f64, t7375: f64, t7378: f64, t8066: f64, t85859: f64, t85963: f64, t86015: f64, t86037: f64, t86076: f64, t86077: f64, t94948: f64, t95000: f64, t95005: f64, t95035: f64) -> (f64, f64) {
    let t103581 = t5415 * t6794 * t131 * t467;
    let t103593 = t29734 * t7376 * t607;
    let t103610 = t85853 * t29754;
    let t103615 = t6224 * t3032;
    let t103624 = 0.80418998823691070228e-1_f64 * t103581 * t7378 + 0.82246703342411321825e-2_f64 * t7373 * t7375 * t19173 * t7376 - 0.10966227112321509577e-1_f64 * t86037 * t94948 * t29734 * t27638 - 0.10966227112321509577e-1_f64 * t24849 * t86015 * t103593 + 0.73108180748810063846e-2_f64 * t27549 * t27550 * t27551 * t17691 - 0.36554090374405031923e-2_f64 * t27549 * t24788 * t29776 + 0.54831135561607547884e-2_f64 * t24589 * t95035 * t8066 + 0.73108180748810063845e-2_f64 * t86076 * t86077 * t103593 - 0.27415567780803773942e-2_f64 * t103610 - 0.16449340668482264365e-1_f64 * t24812 * t85859 * t29749 + 0.49348022005446793095e-1_f64 * t85963 * t95000 * t103615 * t18301 - 0.49348022005446793095e-1_f64 * t85963 * t95005 * t103615 * t4978;
    (t103615, t103624)
}
