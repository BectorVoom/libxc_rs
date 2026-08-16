//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2291/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2291<F: Float>(t24574: F, t27389: F, t8074: F, t85917: F, t24826: F, t27511: F, t15394: F, t2127: F, t221: F, t11147: F, t491: F, t1235: F, t12648: F, t12652: F, t14165: F, t14988: F, t15240: F, t24589: F, t24788: F, t24789: F, t27461: F, t27473: F, t27550: F, t27561: F, t3247: F, t3961: F, t7373: F, t7375: F, t7376: F, t94395: F) -> (F, F, F) {
    let t94779 = F::cast_from(0.18277045187202515961e-2_f64) * t24574 * t27389;
    let t94784 = t85917 * t8074;
    let t94787 = F::cast_from(0.54831135561607547884e-2_f64) * t24826 * t27511;
    let t94796 = t2127 * t221 * t15394;
    let t94797 = t491 * t11147;
    let t94820 = F::cast_from(0.82246703342411321825e-2_f64) * t7373 * t7375 * t15240 * t7376 - F::cast_from(0.18277045187202515961e-2_f64) * t94784 + t94787 + F::cast_from(0.54831135561607547884e-2_f64) * t24589 * t24788 * t27461 + F::cast_from(0.16449340668482264365e-1_f64) * t7373 * t7375 * t14988 * t7376 - F::cast_from(0.8529287754027840782e-2_f64) * t94796 * t27550 * t94797 * t14165 - F::cast_from(0.14621636149762012769e-1_f64) * t94395 * t24789 + F::cast_from(0.54831135561607547884e-2_f64) * t24589 * t24788 * t27473 - F::cast_from(0.10966227112321509577e-1_f64) * t24589 * t27550 * t1235 * t3247 * t3961 - F::cast_from(0.10966227112321509577e-1_f64) * t24589 * t27550 * t27561 * t12652 - F::cast_from(0.54831135561607547884e-2_f64) * t24589 * t27550 * t27561 * t12648;
    (t94779, t94796, t94820)
}
