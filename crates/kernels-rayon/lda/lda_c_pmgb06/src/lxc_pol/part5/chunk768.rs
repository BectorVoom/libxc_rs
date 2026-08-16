//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 768/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk768(t3311: f64, t3316: f64, t3320: f64, t3324: f64, t3327: f64, t3328: f64, t3331: f64, t3335: f64, t5354: f64, t5356: f64, t5363: f64, t5674: f64, t5675: f64, t6586: f64, t6588: f64, t6590: f64) -> f64 {
    let t7217 = t5354 - t5356 - t5363 + t5674 + 16.0_f64 / 3.0_f64 * t5675 - t3311 + 0.21642082724729686_f64 * t3316 + 0.011181742741110338_f64 * t3320 + t3324 + t3327 + 0.07214027574909895_f64 * t3328 + t3331 - t3335 - t6586 - t6588 - t6590;
    t7217
}
