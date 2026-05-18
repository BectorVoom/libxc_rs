//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 684/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk684<F: Float>(t247: F, t701: F, t1126: F, t395: F, t1167: F, t1200: F, t123: F, t199: F, t305: F, t3974: F, t4209: F, t4252: F, t4254: F, t4257: F, t4261: F, t4264: F, t4267: F, t4269: F, t4283: F, t566: F, t726: F, t81: F) -> (F, F, F) {
    let t4284 = t247 * t701;
    let t4286 = t395 * t1126;
    let t4290 = t4252 - F::new(0.42447554366239165) * t4254 - F::new(0.42447554366239165) * t4257 + F::new(0.15917832887339686) * t4261 + F::new(0.3183566577467937) * t4264 + F::new(0.15917832887339686) * t4267 - F::new(0.031835665774679375) * t123 * t4269 * t199 - F::new(0.09550699732403813) * t123 * t1167 * t566 - F::new(0.09550699732403813) * t123 * t726 * t1200 - F::new(0.031835665774679375) * t123 * t305 * t4209 - t4283 + F::new(0.9598512193592288) * t4284 - F::new(0.31995040645307626) * t4286 + F::new(0.05332506774217938) * t81 * t3974;
    (t4284, t4286, t4290)
}
