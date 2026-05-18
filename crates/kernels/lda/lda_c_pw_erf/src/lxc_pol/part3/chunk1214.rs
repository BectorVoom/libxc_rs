//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1214/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1214<F: Float>(t4804: F, t4959: F, t3794: F, t1278: F, t1325: F, t4956: F, t4957: F, t1341: F, t5327: F, t2171: F, t3724: F, t4953: F) -> (F, F, F, F, F, F) {
    let t14321 = F::new(8.0) / F::new(5.0) * t4804 * t4959;
    let t14323 = F::new(8.0) / F::new(5.0) * t3794 * t4959;
    let t14327 = F::new(4.0) / F::new(5.0) * t1325 * t4956 * t4957 * t1278;
    let t14329 = F::new(8.0) / F::new(15.0) * t5327 * t1341;
    let t14331 = F::new(8.0) / F::new(9.0) * t2171 * t3724;
    let t14333 = F::new(8.0) / F::new(5.0) * t4804 * t4953;
    (t14321, t14323, t14327, t14329, t14331, t14333)
}
