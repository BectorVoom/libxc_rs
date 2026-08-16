//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2302/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2302<F: Float>(t24826: F, t29782: F, t29736: F, t86094: F, t17635: F, t17686: F, t17691: F, t24589: F, t24788: F, t24849: F, t24851: F, t27507: F, t27521: F, t27526: F, t27549: F, t27550: F, t27551: F, t27558: F, t27561: F, t27563: F, t29758: F, t29762: F, t72164: F, t7376: F, t94395: F, t94920: F, t95092: F) -> F {
    let t103546 = t24826 * t29782;
    let t103573 = t86094 * t29736;
    let t103577 = F::cast_from(0.27415567780803773942e-2_f64) * t24589 * t24788 * t29758 - F::cast_from(0.54831135561607547884e-2_f64) * t24589 * t27550 * t27561 * t17635 + F::cast_from(0.54831135561607547883e-2_f64) * t103546 - F::cast_from(0.10966227112321509577e-1_f64) * t24589 * t27550 * t27561 * t17691 + F::cast_from(0.54831135561607547884e-2_f64) * t24589 * t24788 * t29762 - F::cast_from(0.16449340668482264365e-1_f64) * t24589 * t27550 * t27551 * t17686 + F::cast_from(0.21932454224643019154e-1_f64) * t27549 * t27550 * t94920 * t17686 - F::cast_from(0.14621636149762012769e-1_f64) * t94395 * t27558 + F::cast_from(0.29243272299524025538e-1_f64) * t94395 * t27563 - F::cast_from(0.27415567780803773942e-2_f64) * t24849 * t24851 * t72164 * t7376 + F::cast_from(0.14621636149762012769e-1_f64) * t95092 * t27526 - F::cast_from(0.18277045187202515961e-2_f64) * t103573 - F::cast_from(0.43864908449286038306e-1_f64) * t27507 * t27521;
    t103577
}
