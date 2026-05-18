//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1132/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1132<F: Float>(t13252: F, t3863: F, t4769: F, t571: F, t4872: F, t4819: F, t9278: F, t1472: F, t5307: F, t1308: F, t1333: F, t2065: F, t951: F) -> (F, F, F, F, F, F) {
    let t13253 = F::new(16.0) / F::new(45.0) * t13252;
    let t13255 = t571 * t3863 * t4769;
    let t13256 = F::new(16.0) / F::new(45.0) * t13255;
    let t13258 = t571 * t3863 * t4872;
    let t13259 = F::new(8.0) / F::new(45.0) * t13258;
    let t13261 = t571 * t9278 * t4819;
    let t13262 = F::new(8.0) / F::new(27.0) * t13261;
    let t13264 = F::new(8.0) / F::new(15.0) * t1472 * t5307;
    let t13269 = F::new(8.0) / F::new(15.0) * t571 * t1308 * t2065 * t1333 * t951;
    (t13253, t13256, t13259, t13262, t13264, t13269)
}
