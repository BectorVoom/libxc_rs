//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 713/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk713(t125: f64, t4463: f64, t1167: f64, t1200: f64, t123: f64, t1808: f64, t199: f64, t2285: f64, t305: f64, t4427: f64, t4431: f64, t4435: f64, t4441: f64, t4444: f64, t4457: f64, t4460: f64, t566: f64, t726: f64, t868: f64, t912: f64) -> (f64, f64) {
    let t4464 = t125 * t4463;
    let t4471 = -0.14149184788746388_f64 * t4427 - 0.14149184788746388_f64 * t4431 - 0.031835665774679375_f64 * t123 * t305 * t4435 + t4441 + t4444 - 0.031835665774679375_f64 * t123 * t1167 * t868 - 0.06367133154935875_f64 * t123 * t726 * t1808 - 0.031835665774679375_f64 * t123 * t912 * t1200 + t4457 + t4460 - 0.031835665774679375_f64 * t123 * t4464 * t199 - 0.06367133154935875_f64 * t123 * t2285 * t566;
    (t4464, t4471)
}
