//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1093/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1093<F: Float>(t16556: F, t2386: F, t851: F, t529: F, t13064: F, t5138: F, t337: F, t12529: F, t12530: F, t1: F, t6560: F, t12537: F, t5139: F) -> (F, F, F, F, F, F, F) {
    let t20146 = F::new(8.0) / F::new(45.0) * t16556;
    let t20147 = t2386 * t851;
    let t20148 = t20147 * t529;
    let t20151 = F::new(2.0) / F::new(9.0) * t5138 * t13064 * t20148;
    let t20152 = t20147 * t337;
    let t20155 = F::new(8.0) / F::new(27.0) * t12529 * t12530 * t20152;
    let t20156 = t6560 * t1;
    let t20159 = F::new(4.0) / F::new(9.0) * t12537 * t5139 * t20156;
    (t20146, t20148, t20151, t20152, t20155, t20156, t20159)
}
