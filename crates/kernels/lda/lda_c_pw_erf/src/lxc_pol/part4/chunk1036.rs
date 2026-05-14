//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1036/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1036<F: Float>(t219: F, t9408: F, t3787: F, t4937: F, t519: F, t10162: F, t1325: F, t2167: F, t5381: F, t3794: F, t4946: F, t4881: F, t5393: F, t5359: F, t1519: F, t1982: F) -> (F, F, F, F, F, F, F, F, F) {
    let t14257 = t9408 * t219;
    let t14277 = t519 * t3787 * t4937;
    let t14313 = t1325 * t10162 * t2167;
    let t14316 = t1325 * t3787 * t5381;
    let t14339 = t3794 * t4946;
    let t14343 = t1325 * t3787 * t4881;
    let t14346 = t1325 * t3787 * t5393;
    let t14349 = t519 * t3787 * t5359;
    let t14351 = t1982 * t1519;
    (t14257, t14277, t14313, t14316, t14339, t14343, t14346, t14349, t14351)
}
