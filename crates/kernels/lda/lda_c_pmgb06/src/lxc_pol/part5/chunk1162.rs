//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1162/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1162<F: Float>(t17276: F, t493: F, t6508: F, t1981: F, t6512: F, t6751: F, t1972: F, t6756: F, t6761: F, t6766: F, t1: F, t1380: F, t6781: F) -> (F, F, F, F, F, F) {
    let t20964 = F::new(8.0) / F::new(27.0) * t493 * t17276 * t6508;
    let t20967 = F::new(4.0) / F::new(9.0) * t1981 * t6751 * t6512;
    let t20969 = t1972 * t6756 / F::new(15.0);
    let t20971 = F::new(2.0) / F::new(15.0) * t1972 * t6761;
    let t20973 = t1972 * t6766 / F::new(9.0);
    let t20977 = F::new(2.0) / F::new(15.0) * t1981 * t1380 * t6781 * t1;
    (t20964, t20967, t20969, t20971, t20973, t20977)
}
