//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 815/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk815(t187: f64, t2342: f64, t2345: f64, t3311: f64, t3313: f64, t3316: f64, t3320: f64, t3324: f64, t3327: f64, t3328: f64, t3331: f64, t3335: f64, t3387: f64, t5361: f64, t5363: f64, t5367: f64) -> f64 {
    let t5674 = 8.0_f64 / 3.0_f64 * t2342 * t187;
    let t5675 = t2345 * t187;
    let t5682 = t5361 - t5363 + t5367 + t5674 + 8.0_f64 / 3.0_f64 * t5675 - t3311 + 0.10821041362364843_f64 * t3313 + 0.4328416544945937_f64 * t3316 + 0.022363485482220676_f64 * t3320 + t3324 + t3327 + 0.1442805514981979_f64 * t3328 + t3331 - t3335 + 4.0_f64 / 3.0_f64 * t3387;
    t5682
}
