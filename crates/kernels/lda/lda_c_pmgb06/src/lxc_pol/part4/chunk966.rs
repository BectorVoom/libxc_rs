//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 966/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk966<F: Float>(t5996: F, t6035: F, t7106: F, t7243: F, t1777: F, t754: F, t936: F, t97: F, t1786: F, t27: F, t2767: F, t749: F) -> (F, F, F) {
    let t7245 = t5996 + t6035 + t7106 + t7243;
    let t8028 = t1777 * t754 * t97 * t936;
    let t8032 = t749 * t1786 * t27 * t2767;
    (t7245, t8028, t8032)
}
