//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1111/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1111<F: Float>(t2146: F, t4820: F, t4763: F, t5272: F, t10463: F, t1325: F, t2392: F, t12695: F, t6229: F, t1251: F, t1313: F, t2497: F, t519: F, t940: F, t3863: F, t571: F, t6286: F) -> (F, F, F, F, F, F) {
    let t16217 = 8.0 / 27.0 * t2146 * t4820;
    let t16219 = 32.0 / 15.0 * t4763 * t5272;
    let t16221 = t1325 * t10463 * t2392;
    let t16222 = 32.0 / 405.0 * t16221;
    let t16224 = t1325 * t12695 * t6229;
    let t16225 = 16.0 / 27.0 * t16224;
    let t16230 = 8.0 / 45.0 * t519 * t1313 * t2497 * t1251 * t940;
    let t16232 = t571 * t3863 * t6286;
    (t16217, t16219, t16222, t16225, t16230, t16232)
}
