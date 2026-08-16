//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1032/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1032<F: Float>(t1325: F, t3859: F, t6322: F, t3794: F, t6292: F, t12695: F, t6454: F, t1639: F, t20: F, t6887: F, t1960: F, t2123: F) -> (F, F, F, F, F) {
    let t17886 = t1325 * t3859 * t6322;
    let t17901 = t3794 * t6292;
    let t17906 = t1325 * t12695 * t6454;
    let t17909 = t6887 * t20 * t1639;
    let t17979 = t1960 * t2123;
    (t17886, t17901, t17906, t17909, t17979)
}
