//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 664/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk664<F: Float>(t123: F, t199: F, t4259: F, t1156: F, t566: F, t1200: F, t722: F, t290: F, t642: F, t247: F, t701: F, t1126: F, t395: F, t2789: F, t301: F, t83: F) -> (F, F, F, F, F, F, F) {
    let t4261 = t123 * t4259 * t199;
    let t4264 = t123 * t1156 * t566;
    let t4267 = t123 * t722 * t1200;
    let t4283 = 1.279801625812305 * t642 * t290;
    let t4284 = t247 * t701;
    let t4286 = t395 * t1126;
    let t4294 = t83 * t2789 * t301;
    (t4261, t4264, t4267, t4283, t4284, t4286, t4294)
}
