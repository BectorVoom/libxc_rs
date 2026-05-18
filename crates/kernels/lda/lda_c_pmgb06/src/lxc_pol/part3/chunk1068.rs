//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1068/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1068<F: Float>(t130: F, t431: F, t5076: F, t5079: F, t1386: F, t1593: F, t2064: F, t5077: F, t1414: F, t1601: F, t1602: F, t337: F, t764: F) -> (F, F, F, F, F, F) {
    let t12683 = t431 * t130;
    let t12684 = t12683 * t5076;
    let t12686 = F::new(4.0) / F::new(15.0) * t12684 * t5079;
    let t12690 = F::new(4.0) / F::new(15.0) * t5077 * t1593 * t2064 * t1386;
    let t12691 = t1601 * t1414;
    let t12693 = t764 * t1602 * t337;
    (t12683, t12684, t12686, t12690, t12691, t12693)
}
