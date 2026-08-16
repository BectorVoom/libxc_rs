//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 754/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk754(t2414: f64, t315: f64, t123: f64, t199: f64, t125: f64, t6716: f64, t2415: f64, t4252: f64, t4254: f64, t4257: f64, t4427: f64, t4431: f64, t4441: f64, t4444: f64, t4457: f64, t4460: f64, t566: f64) -> (f64, f64, f64, f64) {
    let t7113 = t315 * t2414;
    let t7115 = t123 * t7113 * t199;
    let t7117 = t125 * t6716;
    let t7124 = t4252 - 0.14149184788746388_f64 * t4254 - 0.14149184788746388_f64 * t4257 - 0.28298369577492777_f64 * t4431 + t4457 + t4460 - 0.28298369577492777_f64 * t4427 + t4441 + t4444 + 0.053059442957798957_f64 * t7115 - 0.031835665774679375_f64 * t123 * t7117 * t199 - 0.031835665774679375_f64 * t123 * t2415 * t566;
    (t7113, t7115, t7117, t7124)
}
