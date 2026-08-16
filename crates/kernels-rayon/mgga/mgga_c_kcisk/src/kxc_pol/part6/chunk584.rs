//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 584/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk584(t8129: f64, t8158: f64, t1349: f64, t2110: f64, t2209: f64, t338: f64, t3814: f64, t3819: f64, t417: f64, t451: f64, t5641: f64, t5653: f64, t7828: f64, t8099: f64, t8102: f64, t8105: f64, t8108: f64, t8111: f64) -> (f64, f64) {
    let t8159 = t8129 + t8158;
    let t8161 = t3814 + 0.46853067927761790996e-2_f64 * t5641 + 0.93706135855523581992e-2_f64 * t5653 + 0.46853067927761790996e-2_f64 * t3819 * t8099 + 0.93706135855523581992e-2_f64 * t1349 * t8102 - 0.23426533963880895498e-2_f64 * t1349 * t8105 + 0.14055920378328537299e-1_f64 * t417 * t8108 - 0.46853067927761790996e-2_f64 * t417 * t8111 - t7828 * t451 - 2.0_f64 * t2110 * t2209 - t338 * t8159;
    (t8159, t8161)
}
