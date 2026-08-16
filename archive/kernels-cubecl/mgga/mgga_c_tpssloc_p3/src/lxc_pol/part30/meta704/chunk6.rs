//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2303/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2303<F: Float>(t3032: F, t5872: F, t1023: F, t17686: F, t17691: F, t18080: F, t18150: F, t23327: F, t23601: F, t23603: F, t23604: F, t23613: F, t25470: F, t25475: F, t25485: F, t25491: F, t25510: F, t25511: F, t25721: F, t28617: F, t28670: F, t4594: F, t4650: F, t6797: F, t6799: F, t6800: F, t7603: F, t82513: F, t82683: F, t89076: F, t89210: F, t89468: F) -> (F, F) {
    let t100027 = t5872 * t3032;
    let t100068 = -F::cast_from(0.49348022005446793095e-1_f64) * t82513 * t89210 * t100027 * t4594 - F::cast_from(0.16449340668482264365e-1_f64) * t23601 * t25491 * t25485 * t4650 + F::cast_from(0.82246703342411321825e-2_f64) * t23601 * t82683 * t28670 + F::cast_from(0.82246703342411321825e-2_f64) * t82513 * t89468 * t100027 * t1023 - F::cast_from(0.82246703342411321825e-2_f64) * t23601 * t23603 * t18080 * t23604 + F::cast_from(0.16449340668482264365e-1_f64) * t23327 * t25510 * t25721 * t17686 - F::cast_from(0.10966227112321509577e-1_f64) * t23327 * t25510 * t25511 * t17691 + F::cast_from(0.54831135561607547884e-2_f64) * t23327 * t23613 * t28617 - F::cast_from(0.54831135561607547884e-2_f64) * t23327 * t89076 * t7603 - F::cast_from(0.54831135561607547884e-2_f64) * t23327 * t25470 * t25475 + F::cast_from(0.16449340668482264365e-1_f64) * t6797 * t6799 * t18150 * t6800;
    (t100027, t100068)
}
