//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 804/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk804<F: Float>(t3959: F, t3960: F, t3963: F, t4852: F, t4855: F, t4858: F, t4861: F, t4864: F, t4866: F, t4871: F, t4875: F, t4877: F, t4879: F, t4884: F, t4889: F, t4891: F, t4897: F) -> (F,) {
    let t5853 = -t4852 - t4855 - t4858 - t4861 - t4864 + t4866 - t4871 - t4875 - t3959 + 0.06649088888888889 * t3960 + t3963 - t4877 + t4879 - t4884 + t4889 + t4891 + t4897;
    (t5853,)
}
