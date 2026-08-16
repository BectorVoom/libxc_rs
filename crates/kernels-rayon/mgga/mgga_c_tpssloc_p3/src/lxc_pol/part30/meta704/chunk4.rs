//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2301/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2301(t28648: f64, t82431: f64, t28667: f64, t82736: f64, t23665: f64, t28626: f64, t18080: f64, t18161: f64, t23327: f64, t23601: f64, t23670: f64, t23677: f64, t23678: f64, t25470: f64, t25717: f64, t6797: f64, t6799: f64, t6800: f64, t82402: f64, t82534: f64, t88992: f64, t88998: f64) -> f64 {
    let t99960 = t82431 * t28648;
    let t99966 = t82736 * t28667;
    let t99977 = t23665 * t28626;
    let t99983 = -0.18277045187202515961e-2_f64 * t99960 + 0.14621636149762012769e-1_f64 * t82402 * t28648 - 0.43864908449286038307e-1_f64 * t82534 * t28667 + 0.54831135561607547883e-2_f64 * t99966 - t88992 - 0.54831135561607547884e-2_f64 * t23327 * t25470 * t25717 + 0.82246703342411321825e-2_f64 * t6797 * t6799 * t18161 * t6800 - t88998 - 0.43864908449286038307e-1_f64 * t23670 * t28626 + 0.54831135561607547883e-2_f64 * t99977 + 0.16449340668482264365e-1_f64 * t23601 * t23677 * t18080 * t23678;
    t99983
}
