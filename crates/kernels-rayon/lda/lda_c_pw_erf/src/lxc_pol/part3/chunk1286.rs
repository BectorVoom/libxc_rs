//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1286/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1286(t12889: f64, t12891: f64, t12893: f64, t12895: f64, t12897: f64, t12900: f64, t12902: f64, t12903: f64, t12907: f64, t12909: f64, t12913: f64, t12915: f64, t12919: f64) -> f64 {
    let t15053 = t12889 - t12891 + t12893 + t12895 - t12897 - t12900 + t12902 - t12903 + t12907 + t12909 - t12913 - t12915 - t12919;
    t15053
}
