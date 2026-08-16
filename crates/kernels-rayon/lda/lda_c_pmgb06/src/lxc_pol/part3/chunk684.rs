//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 684/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk684(t247: f64, t701: f64, t1126: f64, t395: f64, t1167: f64, t1200: f64, t123: f64, t199: f64, t305: f64, t3974: f64, t4209: f64, t4252: f64, t4254: f64, t4257: f64, t4261: f64, t4264: f64, t4267: f64, t4269: f64, t4283: f64, t566: f64, t726: f64, t81: f64) -> (f64, f64, f64) {
    let t4284 = t247 * t701;
    let t4286 = t395 * t1126;
    let t4290 = t4252 - 0.42447554366239165_f64 * t4254 - 0.42447554366239165_f64 * t4257 + 0.15917832887339686_f64 * t4261 + 0.3183566577467937_f64 * t4264 + 0.15917832887339686_f64 * t4267 - 0.031835665774679375_f64 * t123 * t4269 * t199 - 0.09550699732403813_f64 * t123 * t1167 * t566 - 0.09550699732403813_f64 * t123 * t726 * t1200 - 0.031835665774679375_f64 * t123 * t305 * t4209 - t4283 + 0.9598512193592288_f64 * t4284 - 0.31995040645307626_f64 * t4286 + 0.05332506774217938_f64 * t81 * t3974;
    (t4284, t4286, t4290)
}
