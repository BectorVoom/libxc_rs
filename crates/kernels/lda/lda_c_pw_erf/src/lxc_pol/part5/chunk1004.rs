//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1004/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1004<F: Float>(t558: F, t6865: F, t2146: F, t5371: F, t2549: F, t3783: F, t519: F, t10162: F, t1325: F, t2557: F, t13172: F, t6692: F) -> (F, F, F, F, F) {
    let t15931 = t6865 * t558;
    let t15943 = t2146 * t5371;
    let t15960 = t519 * t3783 * t2549;
    let t15963 = t1325 * t10162 * t2557;
    let t15966 = t1325 * t13172 * t6692;
    (t15931, t15943, t15960, t15963, t15966)
}
