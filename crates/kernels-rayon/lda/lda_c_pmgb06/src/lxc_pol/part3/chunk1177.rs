//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1177/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1177(t3050: f64, t802: f64, t405: f64, t4892: f64, t4889: f64, t4902: f64, t4906: f64, t4913: f64, t1464: f64, t524: f64, t2911: f64, t3357: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t14068 = t802 * t3050;
    let t14073 = t405 * t4892;
    let t14078 = t405 * t4889;
    let t14080 = t405 * t4902;
    let t14082 = t4913 * t4906;
    let t14106 = t524 * t1464;
    let t14110 = t3357 * t2911;
    (t14068, t14073, t14078, t14080, t14082, t14106, t14110)
}
