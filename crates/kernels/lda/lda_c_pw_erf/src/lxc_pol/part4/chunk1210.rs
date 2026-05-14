//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1210/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1210<F: Float>(t13540: F, t2131: F, t5215: F, t13542: F, t13544: F, t13548: F, t13550: F, t16559: F, t352: F, t4506: F, t4515: F, t4522: F, t13115: F, t17864: F, t3976: F, t549: F) -> (F, F, F, F, F, F, F, F, F) {
    let t17923 = 16.0 / 45.0 * t13540;
    let t17925 = 16.0 / 15.0 * t5215 * t2131;
    let t17926 = 16.0 / 45.0 * t13542;
    let t17927 = 16.0 / 45.0 * t13544;
    let t17928 = 16.0 / 45.0 * t13548;
    let t17929 = 16.0 / 135.0 * t13550;
    let t17930 = t16559 * t352;
    let t17933 = 32.0 / 45.0 * t4506 * t4515 * t17930;
    let t17936 = 16.0 / 27.0 * t4506 * t4522 * t17930;
    let t17940 = 64.0 / 45.0 * t13115 * t3976 * t17864 * t549;
    (t17923, t17925, t17926, t17927, t17928, t17929, t17933, t17936, t17940)
}
