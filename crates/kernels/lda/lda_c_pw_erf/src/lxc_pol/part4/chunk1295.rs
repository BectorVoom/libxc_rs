//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1295/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1295<F: Float>(t14992: F, t16519: F, t16521: F, t16526: F, t16531: F, t16532: F, t16534: F, t16535: F, t16536: F, t16538: F, t16543: F, t16548: F, t16551: F, t16553: F, t16558: F, t16563: F, t16565: F) -> (F,) {
    let t19183 = -t16519 - t16521 - t16526 + t16531 + t16532 - 0.13298177777777778 * t14992 + t16534 - t16535 + t16536 - t16538 + t16543 + t16548 - t16551 - t16553 - t16558 - t16563 - t16565;
    (t19183,)
}
