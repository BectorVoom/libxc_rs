//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 736/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk736(t2090: f64, t486: f64, t2885: f64, t851: f64, t166: f64, t161: f64, t1499: f64, t853: f64, t2101: f64, t1588: f64, t831: f64, t2066: f64, t432: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4952 = t486 * t2090 / 15.0_f64;
    let t4953 = t2885 * t851;
    let t4954 = t166 * t4953;
    let t4956 = t161 * t4954 / 30.0_f64;
    let t4958 = t1499 * t853 / 30.0_f64;
    let t4960 = t486 * t2101 / 15.0_f64;
    let t4962 = t831 * t1588 / 30.0_f64;
    let t4964 = t432 * t2066 / 15.0_f64;
    (t4952, t4953, t4954, t4956, t4958, t4960, t4962, t4964)
}
