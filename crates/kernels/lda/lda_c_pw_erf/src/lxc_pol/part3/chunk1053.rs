//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1053/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1053<F: Float>(t10162: F, t1325: F, t2167: F, t3787: F, t5381: F, t3794: F, t4953: F, t4804: F, t4959: F, t1278: F, t4956: F, t4957: F, t1341: F, t5327: F, t2171: F, t3724: F) -> (F, F, F, F, F, F, F, F) {
    let t14313 = t1325 * t10162 * t2167;
    let t14314 = 8.0 / 45.0 * t14313;
    let t14316 = t1325 * t3787 * t5381;
    let t14317 = 8.0 / 15.0 * t14316;
    let t14319 = 8.0 / 5.0 * t3794 * t4953;
    let t14321 = 8.0 / 5.0 * t4804 * t4959;
    let t14323 = 8.0 / 5.0 * t3794 * t4959;
    let t14327 = 4.0 / 5.0 * t1325 * t4956 * t4957 * t1278;
    let t14329 = 8.0 / 15.0 * t5327 * t1341;
    let t14331 = 8.0 / 9.0 * t2171 * t3724;
    (t14314, t14317, t14319, t14321, t14323, t14327, t14329, t14331)
}
