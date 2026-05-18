//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 387/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk387<F: Float>(t1112: F, t247: F, t251: F, t639: F, t652: F, t256: F, t19: F, t465: F, t644: F) -> (F, F, F, F, F, F) {
    let t1415 = t1112 * t247;
    let t1416 = t1415 * t251;
    let t1419 = t639 * t652;
    let t1420 = t1419 * t256;
    let t1422 = t465 * t19;
    let t1423 = t1422 * t644;
    (t1415, t1416, t1419, t1420, t1422, t1423)
}
