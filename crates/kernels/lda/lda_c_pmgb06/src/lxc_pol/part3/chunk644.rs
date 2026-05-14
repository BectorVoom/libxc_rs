//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 644/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk644<F: Float>(t123: F, t199: F, t4259: F, t1156: F, t566: F, t1200: F, t722: F, t125: F, t2803: F, t290: F, t642: F, t247: F, t701: F, t1126: F, t395: F, t1167: F, t305: F, t3974: F, t4209: F, t4252: F, t4254: F, t4257: F, t726: F, t81: F) -> (F, F, F, F, F, F, F, F) {
    let t4261 = t123 * t4259 * t199;
    let t4264 = t123 * t1156 * t566;
    let t4267 = t123 * t722 * t1200;
    let t4269 = t125 * t2803;
    let t4283 = 1.279801625812305 * t642 * t290;
    let t4284 = t247 * t701;
    let t4286 = t395 * t1126;
    let t4290 = t4252 - 0.42447554366239165 * t4254 - 0.42447554366239165 * t4257 + 0.15917832887339686 * t4261 + 0.3183566577467937 * t4264 + 0.15917832887339686 * t4267 - 0.031835665774679375 * t123 * t4269 * t199 - 0.09550699732403813 * t123 * t1167 * t566 - 0.09550699732403813 * t123 * t726 * t1200 - 0.031835665774679375 * t123 * t305 * t4209 - t4283 + 0.9598512193592288 * t4284 - 0.31995040645307626 * t4286 + 0.05332506774217938 * t81 * t3974;
    (t4261, t4264, t4267, t4269, t4283, t4284, t4286, t4290)
}
