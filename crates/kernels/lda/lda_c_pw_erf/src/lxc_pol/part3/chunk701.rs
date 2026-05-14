//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 701/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk701<F: Float>(t4907: F, t519: F, t4855: F, t4858: F, t4861: F, t4864: F, t4866: F, t4871: F, t4875: F, t4877: F, t4879: F, t4884: F, t4889: F, t4891: F, t4897: F, t4899: F, t4903: F, t4905: F) -> (F, F) {
    let t4908 = t519 * t4907;
    let t4909 = 8.0 / 27.0 * t4908;
    let t4910 = -t4855 - t4858 - t4861 - t4864 + t4866 - t4871 - t4875 - t4877 + t4879 - t4884 + t4889 + t4891 + t4897 - t4899 + t4903 + t4905 - t4909;
    (t4909, t4910)
}
