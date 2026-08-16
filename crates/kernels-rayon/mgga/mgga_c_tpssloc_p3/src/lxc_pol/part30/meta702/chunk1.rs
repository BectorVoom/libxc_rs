//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2274/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2274(t225: f64, t28488: f64, t10164: f64, t1066: f64, t14545: f64, t14555: f64, t1599: f64, t17575: f64, t17588: f64, t1921: f64, t23365: f64, t23588: f64, t25757: f64, t25801: f64, t25810: f64, t28485: f64, t28495: f64, t3169: f64, t387: f64, t4540: f64, t4664: f64, t5838: f64, t6687: f64, t6776: f64, t7600: f64, t7624: f64, t7625: f64, t88731: f64, t88753: f64) -> f64 {
    let t99248 = t28488 * t225;
    let t99271 = -12.0_f64 * t25757 * t10164 * t7624 * t4664 - 0.12184696791468343974e-2_f64 * t88731 - 2.0_f64 * t14545 * t7625 + 4.0_f64 * t17588 * t6776 - 2.0_f64 * t99248 * t1066 + 4.0_f64 * t14555 * t7600 + 0.16449340668482264365e-1_f64 * t6687 * t23365 * t28495 + 0.82246703342411321825e-2_f64 * t6687 * t5838 * t23588 + 2.0_f64 * t17575 * t6776 - 0.16449340668482264365e-1_f64 * t6687 * t1599 * t1921 * t387 * t4540 + 4.0_f64 * t3169 * t28485 + 0.54831135561607547884e-2_f64 * t6687 * t25810 * t25801 - t88753;
    t99271
}
