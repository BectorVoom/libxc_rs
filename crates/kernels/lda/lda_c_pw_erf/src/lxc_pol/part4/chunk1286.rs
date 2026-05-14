//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1286/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1286<F: Float>(t15919: F, t15921: F, t15923: F, t15925: F, t15928: F, t15930: F, t15935: F, t15937: F, t15939: F, t15941: F, t15942: F, t15944: F, t15946: F, t15948: F, t15950: F, t15956: F, t15958: F) -> (F,) {
    let t19117 = t15919 - t15921 + t15923 - t15925 - t15928 - t15930 + t15935 + t15937 + t15939 - t15941 - t15942 + t15944 + t15946 + t15948 - t15950 + t15956 + t15958;
    (t19117,)
}
