//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2270/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2270(t1625: f64, t7577: f64, t14552: f64, t1604: f64, t17691: f64, t23327: f64, t23329: f64, t254: f64, t25423: f64, t25424: f64, t25429: f64, t25431: f64, t25442: f64, t25750: f64, t25759: f64, t25801: f64, t25815: f64, t28701: f64, t4342: f64, t6691: f64, t7553: f64, t7625: f64, t82502: f64, t88050: f64, t88058: f64, t88096: f64, t88112: f64, t88162: f64, t99070: f64) -> f64 {
    let t99131 = t7577 * t1625;
    let t99143 = -0.54831135561607547884e-2_f64 * t23327 * t25442 * t25801 + 0.10966227112321509577e-1_f64 * t23327 * t88112 * t4342 * t99070 + 0.54831135561607547884e-2_f64 * t23327 * t82502 * t28701 + 0.10966227112321509577e-1_f64 * t23327 * t88162 * t25424 - 0.73108180748810063845e-2_f64 * t25429 * t88162 * t25431 - 0.54831135561607547884e-2_f64 * t23327 * t88058 * t7553 + 0.54831135561607547884e-2_f64 * t23327 * t88162 * t25815 - 0.10966227112321509577e-1_f64 * t23327 * t23329 * t25423 * t17691 - 0.54831135561607547883e-2_f64 * t23327 * t99131 * t6691 - 0.54831135561607547883e-2_f64 * t23327 * t88050 * t25750 + t88096 - 12.0_f64 * t1604 * t254 * t25759 - 2.0_f64 * t14552 * t7625;
    t99143
}
