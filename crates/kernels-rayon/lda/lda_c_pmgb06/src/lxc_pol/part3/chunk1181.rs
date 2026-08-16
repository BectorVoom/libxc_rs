//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1181/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1181(t13560: f64, t2085: f64, t2060: f64, t848: f64, t2082: f64, t955: f64, t2079: f64, t405: f64, t4848: f64, t4853: f64, t4913: f64, t4899: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t14162 = t13560 * t2085;
    let t14170 = t2060 * t848;
    let t14181 = t955 * t2082;
    let t14183 = t955 * t2079;
    let t14185 = t405 * t4848;
    let t14187 = t4913 * t4853;
    let t14189 = t405 * t4899;
    (t14162, t14170, t14181, t14183, t14185, t14187, t14189)
}
