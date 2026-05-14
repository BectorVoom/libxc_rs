//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 847/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk847<F: Float>(t176: F, t1988: F, t1826: F, t493: F, t4588: F, t1821: F, t2549: F, t529: F, t1380: F, t1414: F, t2389: F, t337: F, t1915: F, t1464: F, t1919: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t6747 = t1988 * t176;
    let t6748 = t6747 * t1826;
    let t6750 = 4.0 / 45.0 * t493 * t6748;
    let t6751 = t4588 * t176;
    let t6752 = t6751 * t1821;
    let t6754 = 2.0 / 27.0 * t493 * t6752;
    let t6755 = t2549 * t529;
    let t6756 = t1380 * t6755;
    let t6758 = t493 * t6756 / 45.0;
    let t6759 = t1414 * t2389;
    let t6760 = t6759 * t337;
    let t6761 = t1915 * t6760;
    let t6763 = 2.0 / 45.0 * t493 * t6761;
    let t6764 = t1464 * t2389;
    let t6765 = t6764 * t337;
    let t6766 = t1919 * t6765;
    (t6747, t6748, t6750, t6751, t6752, t6754, t6755, t6756, t6758, t6759, t6760, t6761, t6763, t6764, t6765, t6766)
}
