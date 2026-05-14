//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 377/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk377<F: Float>(t1476: F, t1821: F, t36: F, t1414: F, t764: F, t337: F) -> (F, F, F, F) {
    let t1822 = t1476 * t1821;
    let t1823 = t36 * t1822;
    let t1825 = t1414 * t764;
    let t1826 = t1825 * t337;
    (t1822, t1823, t1825, t1826)
}
