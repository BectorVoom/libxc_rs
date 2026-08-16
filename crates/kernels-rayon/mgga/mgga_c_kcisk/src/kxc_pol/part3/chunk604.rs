//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 604/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk604(t5139: f64, t5171: f64, t1689: f64, t1809: f64, t1860: f64, t4794: f64, t5084: f64, t5085: f64, t5087: f64, t5089: f64, t5090: f64, t5094: f64, t5097: f64, t5102: f64, t5105: f64, t604: f64, t674: f64, t702: f64) -> (f64, f64) {
    let t5172 = t5139 + t5171;
    let t5174 = t5084 + 0.46853067927761790996e-2_f64 * t5085 + 0.93706135855523581992e-2_f64 * t5087 + 0.46853067927761790996e-2_f64 * t5089 * t5090 + 0.93706135855523581992e-2_f64 * t1809 * t5094 - 0.23426533963880895498e-2_f64 * t1809 * t5097 + 0.14055920378328537299e-1_f64 * t674 * t5102 - 0.46853067927761790996e-2_f64 * t674 * t5105 - t4794 * t702 - 2.0_f64 * t1689 * t1860 - t604 * t5172;
    (t5172, t5174)
}
