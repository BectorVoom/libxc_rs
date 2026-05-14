//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1212/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1212<F: Float>(t16883: F, t4506: F, t4522: F, t17923: F, t17925: F, t17926: F, t17927: F, t17928: F, t17929: F, t17933: F, t17936: F, t17940: F, t17943: F, t17947: F, t17950: F, t17953: F, t17956: F, t17958: F, t17960: F) -> (F, F) {
    let t17963 = 8.0 / 27.0 * t4506 * t4522 * t16883;
    let t17964 = t17923 + t17925 + t17926 + t17927 + t17928 + t17929 + t17933 - t17936 + t17940 + t17943 + t17947 - t17950 - t17953 - t17956 - t17958 + t17960 - t17963;
    (t17963, t17964)
}
