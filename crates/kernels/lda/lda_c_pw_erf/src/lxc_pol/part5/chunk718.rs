//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 718/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk718<F: Float>(t348: F, t6426: F, t3806: F, t519: F, t784: F, t806: F, t542: F, t5289: F, t1325: F, t2031: F, t2171: F, t1987: F) -> (F, F, F, F, F, F, F, F, F) {
    let t6427 = t6426 * t348;
    let t6428 = t3806 * t6427;
    let t6430 = F::new(8.0) / F::new(45.0) * t519 * t6428;
    let t6431 = t784 * t806;
    let t6432 = t6431 * t542;
    let t6433 = t5289 * t6432;
    let t6435 = F::new(16.0) / F::new(15.0) * t1325 * t6433;
    let t6437 = F::new(8.0) / F::new(45.0) * t2171 * t2031;
    let t6439 = F::new(16.0) / F::new(45.0) * t2171 * t1987;
    (t6427, t6428, t6430, t6431, t6432, t6433, t6435, t6437, t6439)
}
