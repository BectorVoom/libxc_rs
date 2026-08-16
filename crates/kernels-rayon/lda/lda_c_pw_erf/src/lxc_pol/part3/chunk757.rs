//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 757/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk757(t1124: f64, t521: f64, t2177: f64, t519: f64, t4855: f64, t4858: f64, t4861: f64, t4864: f64, t4866: f64, t4871: f64, t4875: f64, t4877: f64, t4879: f64, t4884: f64, t4889: f64, t4891: f64, t4897: f64, t4899: f64, t4903: f64, t4905: f64) -> (f64, f64, f64, f64) {
    let t4906 = t1124 * t521;
    let t4907 = t4906 * t2177;
    let t4908 = t519 * t4907;
    let t4909 = 8.0_f64 / 27.0_f64 * t4908;
    let t4910 = -t4855 - t4858 - t4861 - t4864 + t4866 - t4871 - t4875 - t4877 + t4879 - t4884 + t4889 + t4891 + t4897 - t4899 + t4903 + t4905 - t4909;
    (t4906, t4907, t4909, t4910)
}
