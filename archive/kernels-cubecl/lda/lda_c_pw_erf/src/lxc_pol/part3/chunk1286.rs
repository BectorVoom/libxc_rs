//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1286/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1286<F: Float>(t12889: F, t12891: F, t12893: F, t12895: F, t12897: F, t12900: F, t12902: F, t12903: F, t12907: F, t12909: F, t12913: F, t12915: F, t12919: F) -> F {
    let t15053 = t12889 - t12891 + t12893 + t12895 - t12897 - t12900 + t12902 - t12903 + t12907 + t12909 - t12913 - t12915 - t12919;
    t15053
}
