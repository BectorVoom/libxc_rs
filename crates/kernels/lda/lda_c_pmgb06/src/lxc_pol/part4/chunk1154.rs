//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1154/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1154<F: Float>(t15196: F, t2002: F, t5345: F, t1080: F, t6764: F, t1915: F, t493: F, t10139: F, t1602: F, t2541: F, t1447: F, t6518: F) -> (F, F, F, F, F, F) {
    let t15197 = F::new(8.0) / F::new(135.0) * t15196;
    let t15199 = F::new(4.0) / F::new(45.0) * t2002 * t5345;
    let t15200 = t6764 * t1080;
    let t15203 = F::new(2.0) / F::new(15.0) * t493 * t1915 * t15200;
    let t15207 = F::new(2.0) / F::new(27.0) * t493 * t10139 * t2541 * t1602;
    let t15208 = t1447 * t6518;
    (t15197, t15199, t15200, t15203, t15207, t15208)
}
