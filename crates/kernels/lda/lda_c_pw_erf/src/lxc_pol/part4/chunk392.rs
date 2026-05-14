//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 392/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk392<F: Float>(t1476: F, t571: F, t575: F, t954: F, t574: F, t212: F, t558: F) -> (F, F, F, F, F, F) {
    let t1477 = t571 * t1476;
    let t1478 = 16.0 / 135.0 * t1477;
    let t1479 = t575 * t954;
    let t1480 = t574 * t1479;
    let t1482 = 4.0 / 45.0 * t571 * t1480;
    let t1484 = 1.0 / t212 / t558;
    (t1477, t1478, t1479, t1480, t1482, t1484)
}
