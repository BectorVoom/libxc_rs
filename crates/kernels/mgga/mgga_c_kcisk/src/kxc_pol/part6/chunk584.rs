//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 584/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk584<F: Float>(t8129: F, t8158: F, t1349: F, t2110: F, t2209: F, t338: F, t3814: F, t3819: F, t417: F, t451: F, t5641: F, t5653: F, t7828: F, t8099: F, t8102: F, t8105: F, t8108: F, t8111: F) -> (F, F) {
    let t8159 = t8129 + t8158;
    let t8161 = t3814 + F::cast_from(0.46853067927761790996e-2_f64) * t5641 + F::cast_from(0.93706135855523581992e-2_f64) * t5653 + F::cast_from(0.46853067927761790996e-2_f64) * t3819 * t8099 + F::cast_from(0.93706135855523581992e-2_f64) * t1349 * t8102 - F::cast_from(0.23426533963880895498e-2_f64) * t1349 * t8105 + F::cast_from(0.14055920378328537299e-1_f64) * t417 * t8108 - F::cast_from(0.46853067927761790996e-2_f64) * t417 * t8111 - t7828 * t451 - F::new(2.0) * t2110 * t2209 - t338 * t8159;
    (t8159, t8161)
}
