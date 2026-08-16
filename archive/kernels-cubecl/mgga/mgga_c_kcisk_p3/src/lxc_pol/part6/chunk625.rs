//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 625/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk625<F: Float>(t8637: F, t8661: F, t1809: F, t2399: F, t2505: F, t5084: F, t5089: F, t604: F, t6729: F, t674: F, t6741: F, t702: F, t8491: F, t8494: F, t8497: F, t8501: F, t8537: F, t8616: F) -> (F, F) {
    let t8662 = t8637 + t8661;
    let t8664 = t5084 + F::cast_from(0.46853067927761790996e-2_f64) * t6729 + F::cast_from(0.93706135855523581992e-2_f64) * t6741 + F::cast_from(0.46853067927761790996e-2_f64) * t5089 * t8491 + F::cast_from(0.93706135855523581992e-2_f64) * t1809 * t8494 - F::cast_from(0.23426533963880895498e-2_f64) * t1809 * t8497 + F::cast_from(0.14055920378328537299e-1_f64) * t674 * t8501 - F::cast_from(0.46853067927761790996e-2_f64) * t674 * t8537 - t8616 * t702 - F::cast_from(2.0_f64) * t2399 * t2505 - t604 * t8662;
    (t8662, t8664)
}
