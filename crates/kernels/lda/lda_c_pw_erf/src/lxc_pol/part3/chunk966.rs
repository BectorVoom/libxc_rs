//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 966/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk966<F: Float>(t2023: F, t3727: F, t1308: F, t352: F, t5029: F, t558: F, t571: F, t1310: F, t5334: F, t1472: F, t4770: F, t3802: F, t519: F, t5243: F, t10463: F, t1972: F) -> (F, F, F, F, F, F) {
    let t12848 = 4.0 / 15.0 * t3727 * t2023;
    let t12853 = 4.0 / 15.0 * t571 * t1308 * t5029 * t558 * t352;
    let t12855 = 8.0 / 15.0 * t5334 * t1310;
    let t12857 = 8.0 / 15.0 * t1472 * t4770;
    let t12859 = t519 * t3802 * t5243;
    let t12860 = 8.0 / 45.0 * t12859;
    let t12862 = t519 * t10463 * t1972;
    (t12848, t12853, t12855, t12857, t12860, t12862)
}
