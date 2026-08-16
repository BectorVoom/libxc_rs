//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2302/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2302(t24826: f64, t29782: f64, t29736: f64, t86094: f64, t17635: f64, t17686: f64, t17691: f64, t24589: f64, t24788: f64, t24849: f64, t24851: f64, t27507: f64, t27521: f64, t27526: f64, t27549: f64, t27550: f64, t27551: f64, t27558: f64, t27561: f64, t27563: f64, t29758: f64, t29762: f64, t72164: f64, t7376: f64, t94395: f64, t94920: f64, t95092: f64) -> f64 {
    let t103546 = t24826 * t29782;
    let t103573 = t86094 * t29736;
    let t103577 = 0.27415567780803773942e-2_f64 * t24589 * t24788 * t29758 - 0.54831135561607547884e-2_f64 * t24589 * t27550 * t27561 * t17635 + 0.54831135561607547883e-2_f64 * t103546 - 0.10966227112321509577e-1_f64 * t24589 * t27550 * t27561 * t17691 + 0.54831135561607547884e-2_f64 * t24589 * t24788 * t29762 - 0.16449340668482264365e-1_f64 * t24589 * t27550 * t27551 * t17686 + 0.21932454224643019154e-1_f64 * t27549 * t27550 * t94920 * t17686 - 0.14621636149762012769e-1_f64 * t94395 * t27558 + 0.29243272299524025538e-1_f64 * t94395 * t27563 - 0.27415567780803773942e-2_f64 * t24849 * t24851 * t72164 * t7376 + 0.14621636149762012769e-1_f64 * t95092 * t27526 - 0.18277045187202515961e-2_f64 * t103573 - 0.43864908449286038306e-1_f64 * t27507 * t27521;
    t103577
}
