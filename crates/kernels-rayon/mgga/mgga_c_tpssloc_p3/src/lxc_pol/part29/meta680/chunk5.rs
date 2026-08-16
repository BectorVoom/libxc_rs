//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2291/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2291(t24574: f64, t27389: f64, t8074: f64, t85917: f64, t24826: f64, t27511: f64, t15394: f64, t2127: f64, t221: f64, t11147: f64, t491: f64, t1235: f64, t12648: f64, t12652: f64, t14165: f64, t14988: f64, t15240: f64, t24589: f64, t24788: f64, t24789: f64, t27461: f64, t27473: f64, t27550: f64, t27561: f64, t3247: f64, t3961: f64, t7373: f64, t7375: f64, t7376: f64, t94395: f64) -> (f64, f64, f64) {
    let t94779 = 0.18277045187202515961e-2_f64 * t24574 * t27389;
    let t94784 = t85917 * t8074;
    let t94787 = 0.54831135561607547884e-2_f64 * t24826 * t27511;
    let t94796 = t2127 * t221 * t15394;
    let t94797 = t491 * t11147;
    let t94820 = 0.82246703342411321825e-2_f64 * t7373 * t7375 * t15240 * t7376 - 0.18277045187202515961e-2_f64 * t94784 + t94787 + 0.54831135561607547884e-2_f64 * t24589 * t24788 * t27461 + 0.16449340668482264365e-1_f64 * t7373 * t7375 * t14988 * t7376 - 0.8529287754027840782e-2_f64 * t94796 * t27550 * t94797 * t14165 - 0.14621636149762012769e-1_f64 * t94395 * t24789 + 0.54831135561607547884e-2_f64 * t24589 * t24788 * t27473 - 0.10966227112321509577e-1_f64 * t24589 * t27550 * t1235 * t3247 * t3961 - 0.10966227112321509577e-1_f64 * t24589 * t27550 * t27561 * t12652 - 0.54831135561607547884e-2_f64 * t24589 * t27550 * t27561 * t12648;
    (t94779, t94796, t94820)
}
