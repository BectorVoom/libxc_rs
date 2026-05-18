//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1153/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1153<F: Float>(t15184: F, t493: F, t5358: F, t5486: F, t1423: F, t6775: F, t1069: F, t1385: F, t1531: F, t2648: F, t439: F, t1908: F, t5220: F) -> (F, F, F, F, F) {
    let t15185 = F::new(8.0) / F::new(135.0) * t15184;
    let t15188 = F::new(4.0) / F::new(45.0) * t493 * t5486 * t5358;
    let t15189 = t1423 * t6775;
    let t15190 = F::new(4.0) / F::new(135.0) * t15189;
    let t15195 = F::new(2.0) / F::new(45.0) * t439 * t1385 * t2648 * t1531 * t1069;
    let t15196 = t5220 * t1908;
    (t15185, t15188, t15190, t15195, t15196)
}
