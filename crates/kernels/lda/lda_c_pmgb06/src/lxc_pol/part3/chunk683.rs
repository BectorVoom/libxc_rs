//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 683/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk683<F: Float>(t1152: F, t123: F, t566: F, t1166: F, t315: F, t199: F, t1156: F, t1200: F, t722: F, t125: F, t2803: F, t290: F, t642: F) -> (F, F, F, F, F, F, F) {
    let t4257 = t123 * t1152 * t566;
    let t4259 = t315 * t1166;
    let t4261 = t123 * t4259 * t199;
    let t4264 = t123 * t1156 * t566;
    let t4267 = t123 * t722 * t1200;
    let t4269 = t125 * t2803;
    let t4283 = F::new(1.279801625812305) * t642 * t290;
    (t4257, t4259, t4261, t4264, t4267, t4269, t4283)
}
