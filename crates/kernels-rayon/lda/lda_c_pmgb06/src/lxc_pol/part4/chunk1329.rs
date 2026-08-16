//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1329/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1329(t17427: f64, t5068: f64, t5069: f64, t13068: f64, t16386: f64, t5138: f64, t13672: f64, t17435: f64, t17430: f64, t17433: f64, t17434: f64, t17438: f64, t17440: f64, t17444: f64, t17448: f64, t17452: f64, t17455: f64, t17460: f64, t17465: f64, t17469: f64) -> (f64, f64, f64, f64) {
    let t17472 = 4.0_f64 / 45.0_f64 * t5068 * t5069 * t17427;
    let t17475 = 4.0_f64 / 9.0_f64 * t5138 * t13068 * t16386;
    let t17478 = 16.0_f64 / 45.0_f64 * t13672 * t5069 * t17435;
    let t17479 = -t17430 - t17433 + t17434 + t17438 + t17440 + t17444 + t17448 + t17452 - t17455 - t17460 - t17465 + t17469 + t17472 + t17475 - t17478;
    (t17472, t17475, t17478, t17479)
}
