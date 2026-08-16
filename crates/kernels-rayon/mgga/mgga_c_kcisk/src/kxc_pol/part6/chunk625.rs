//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 625/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk625(t8637: f64, t8661: f64, t1809: f64, t2399: f64, t2505: f64, t5084: f64, t5089: f64, t604: f64, t6729: f64, t674: f64, t6741: f64, t702: f64, t8491: f64, t8494: f64, t8497: f64, t8501: f64, t8537: f64, t8616: f64) -> (f64, f64) {
    let t8662 = t8637 + t8661;
    let t8664 = t5084 + 0.46853067927761790996e-2_f64 * t6729 + 0.93706135855523581992e-2_f64 * t6741 + 0.46853067927761790996e-2_f64 * t5089 * t8491 + 0.93706135855523581992e-2_f64 * t1809 * t8494 - 0.23426533963880895498e-2_f64 * t1809 * t8497 + 0.14055920378328537299e-1_f64 * t674 * t8501 - 0.46853067927761790996e-2_f64 * t674 * t8537 - t8616 * t702 - 2.0_f64 * t2399 * t2505 - t604 * t8662;
    (t8662, t8664)
}
