//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 826/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk826<F: Float>(t1472: F, t3864: F, t3863: F, t3872: F, t571: F, t3802: F, t3811: F, t519: F, t2070: F, t548: F, t550: F, t1404: F, t1518: F, t211: F, t172: F, t184: F, t4008: F) -> (F, F, F, F, F, F) {
    let t9513 = t1472 * t3864;
    let t9540 = t571 * t3863 * t3872;
    let t9590 = t519 * t3802 * t3811;
    let t9593 = t548 * t2070 * t550;
    let t9596 = t211 * t1518 * t1404;
    let t9599 = t172 * t4008 * t184;
    (t9513, t9540, t9590, t9593, t9596, t9599)
}
