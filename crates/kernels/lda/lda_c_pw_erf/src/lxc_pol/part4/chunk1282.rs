//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1282/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1282<F: Float>(t15594: F, t15599: F, t15601: F, t15603: F, t15605: F, t15609: F, t15611: F, t15613: F, t15616: F, t15618: F, t15623: F, t15625: F, t15627: F, t15629: F, t15631: F, t15634: F, t15639: F) -> (F,) {
    let t19108 = t15594 - t15599 + t15601 - t15603 - t15605 + t15609 - t15611 - t15613 - t15616 - t15618 + t15623 - t15625 - t15627 + t15629 + t15631 + t15634 - t15639;
    (t19108,)
}
