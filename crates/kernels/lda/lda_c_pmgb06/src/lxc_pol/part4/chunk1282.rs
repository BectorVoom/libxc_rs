//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1282/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1282<F: Float>(t12012: F, t1924: F, t493: F, t497: F, t6904: F, t1380: F, t337: F, t2002: F, t5483: F, t1444: F, t6791: F, t9921: F) -> (F, F, F, F, F) {
    let t16855 = F::new(4.0) / F::new(45.0) * t493 * t12012 * t1924;
    let t16856 = t6904 * t497;
    let t16860 = F::new(2.0) / F::new(45.0) * t493 * t1380 * t16856 * t337;
    let t16862 = F::new(4.0) / F::new(45.0) * t2002 * t5483;
    let t16864 = F::new(4.0) / F::new(45.0) * t1444 * t6791;
    let t16865 = F::new(4.0) / F::new(405.0) * t9921;
    (t16855, t16860, t16862, t16864, t16865)
}
