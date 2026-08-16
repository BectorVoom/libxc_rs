//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 837/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk837(t3959: f64, t3960: f64, t3963: f64, t4852: f64, t4855: f64, t4858: f64, t4861: f64, t4864: f64, t4866: f64, t4871: f64, t4875: f64, t4877: f64, t4879: f64, t4884: f64, t4889: f64, t4891: f64, t4897: f64) -> f64 {
    let t5853 = -t4852 - t4855 - t4858 - t4861 - t4864 + t4866 - t4871 - t4875 - t3959 + 0.06649088888888889_f64 * t3960 + t3963 - t4877 + t4879 - t4884 + t4889 + t4891 + t4897;
    t5853
}
