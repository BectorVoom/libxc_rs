//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1066/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1066<F: Float>(t1080: F, t1464: F, t2623: F, t2991: F, t493: F, t1972: F, t5277: F, t5282: F, t439: F, t5267: F, t5482: F, t1074: F, t1385: F, t6773: F, t443: F, t6225: F) -> (F, F, F, F, F, F) {
    let t15923 = t493 * t2991 * t2623 * t1464 * t1080 / 27.0;
    let t15925 = 2.0 / 45.0 * t1972 * t5277;
    let t15927 = 2.0 / 27.0 * t1972 * t5282;
    let t15930 = 2.0 / 45.0 * t439 * t5482 * t5267;
    let t15934 = t439 * t1385 * t6773 * t1074 / 45.0;
    let t15935 = t6225 * t443;
    (t15923, t15925, t15927, t15930, t15934, t15935)
}
