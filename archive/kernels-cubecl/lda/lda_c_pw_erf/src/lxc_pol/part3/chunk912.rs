//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 912/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk912<F: Float>(t348: F, t945: F, t1472: F, t3864: F, t3863: F, t3872: F, t571: F, t3802: F, t3811: F, t519: F, t2070: F, t548: F, t550: F) -> (F, F, F, F, F) {
    let t9481 = t348 * t945;
    let t9513 = t1472 * t3864;
    let t9540 = t571 * t3863 * t3872;
    let t9590 = t519 * t3802 * t3811;
    let t9593 = t548 * t2070 * t550;
    (t9481, t9513, t9540, t9590, t9593)
}
