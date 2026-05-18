//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 817/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk817<F: Float>(t4618: F, t5504: F, t5629: F, t5631: F, t5641: F, t5644: F, t5645: F, t5660: F, t5661: F, t5665: F, t5666: F, t5668: F, t5669: F, t5672: F, t5682: F, t5685: F) -> F {
    let t5689 = t4618 + t5629 + t5631 + t5641 + t5644 + t5645 + t5660 + t5661 + t5665 + t5666 + t5668 + t5669 + t5672 + t5682 + t5685 + t5504;
    t5689
}
