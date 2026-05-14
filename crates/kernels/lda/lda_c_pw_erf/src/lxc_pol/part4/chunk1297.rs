//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1297/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1297<F: Float>(t16623: F, t16625: F, t16627: F, t16634: F, t16637: F, t16640: F, t16643: F, t16647: F, t16649: F, t16651: F, t16653: F, t16656: F, t16660: F, t16665: F, t16667: F, t16669: F, t16673: F) -> (F,) {
    let t19189 = -t16623 + t16625 + t16627 - t16634 - t16637 - t16640 + t16643 + t16647 - t16649 - t16651 + t16653 - t16656 + t16660 - t16665 + t16667 + t16669 - t16673;
    (t19189,)
}
