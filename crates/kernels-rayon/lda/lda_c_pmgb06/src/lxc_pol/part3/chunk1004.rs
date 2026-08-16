//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1004/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1004(t4159: f64, t871: f64, t9402: f64, t11589: f64, t11915: f64, t11918: f64, t11921: f64, t11928: f64, t11930: f64, t11934: f64, t11937: f64, t11940: f64, t11943: f64, t205: f64, t208: f64, t213: f64) -> (f64, f64) {
    let t11944 = t871 * t4159;
    let t11946 = t9402 / 45.0_f64;
    let t11947 = -t11915 - t11918 + t11921 + t11589 * t205 * t208 * t213 / 3.0_f64 + t11928 + 0.18233333333333332_f64 * t11930 + t11934 - t11937 - t11940 - t11943 - 0.06649088888888889_f64 * t11944 - t11946;
    (t11946, t11947)
}
