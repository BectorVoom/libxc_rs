//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2303/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2303(t3032: f64, t5872: f64, t1023: f64, t17686: f64, t17691: f64, t18080: f64, t18150: f64, t23327: f64, t23601: f64, t23603: f64, t23604: f64, t23613: f64, t25470: f64, t25475: f64, t25485: f64, t25491: f64, t25510: f64, t25511: f64, t25721: f64, t28617: f64, t28670: f64, t4594: f64, t4650: f64, t6797: f64, t6799: f64, t6800: f64, t7603: f64, t82513: f64, t82683: f64, t89076: f64, t89210: f64, t89468: f64) -> (f64, f64) {
    let t100027 = t5872 * t3032;
    let t100068 = -0.49348022005446793095e-1_f64 * t82513 * t89210 * t100027 * t4594 - 0.16449340668482264365e-1_f64 * t23601 * t25491 * t25485 * t4650 + 0.82246703342411321825e-2_f64 * t23601 * t82683 * t28670 + 0.82246703342411321825e-2_f64 * t82513 * t89468 * t100027 * t1023 - 0.82246703342411321825e-2_f64 * t23601 * t23603 * t18080 * t23604 + 0.16449340668482264365e-1_f64 * t23327 * t25510 * t25721 * t17686 - 0.10966227112321509577e-1_f64 * t23327 * t25510 * t25511 * t17691 + 0.54831135561607547884e-2_f64 * t23327 * t23613 * t28617 - 0.54831135561607547884e-2_f64 * t23327 * t89076 * t7603 - 0.54831135561607547884e-2_f64 * t23327 * t25470 * t25475 + 0.16449340668482264365e-1_f64 * t6797 * t6799 * t18150 * t6800;
    (t100027, t100068)
}
