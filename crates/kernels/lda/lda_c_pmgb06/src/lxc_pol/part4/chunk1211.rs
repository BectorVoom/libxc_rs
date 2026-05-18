//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1211/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1211<F: Float>(t4602: F, t6395: F, t2496: F, t493: F, t9925: F, t2979: F, t6390: F, t10152: F, t6517: F, t1908: F, t5187: F, t1420: F, t6524: F) -> (F, F, F, F, F, F) {
    let t15962 = F::new(8.0) / F::new(45.0) * t4602 * t6395;
    let t15965 = F::new(2.0) / F::new(45.0) * t493 * t9925 * t2496;
    let t15968 = F::new(4.0) / F::new(45.0) * t493 * t2979 * t6390;
    let t15971 = F::new(4.0) / F::new(45.0) * t493 * t10152 * t6517;
    let t15973 = F::new(4.0) / F::new(45.0) * t5187 * t1908;
    let t15975 = F::new(4.0) / F::new(45.0) * t1420 * t6524;
    (t15962, t15965, t15968, t15971, t15973, t15975)
}
