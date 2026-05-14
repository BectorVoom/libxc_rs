//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1010/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1010<F: Float>(t3859: F, t4628: F, t519: F, t5295: F, t9304: F, t10467: F, t1996: F, t3802: F, t5425: F, t5243: F, t10463: F, t1972: F, t10313: F, t1967: F, t197: F, t518: F, t5210: F) -> (F, F, F, F, F, F, F, F) {
    let t12820 = t519 * t3859 * t4628;
    let t12831 = t519 * t9304 * t5295;
    let t12838 = t519 * t10467 * t1996;
    let t12841 = t519 * t3802 * t5425;
    let t12859 = t519 * t3802 * t5243;
    let t12862 = t519 * t10463 * t1972;
    let t12869 = t519 * t10313 * t197 * t1967;
    let t12874 = t5210 * t518;
    (t12820, t12831, t12838, t12841, t12859, t12862, t12869, t12874)
}
