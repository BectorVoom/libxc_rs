//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 735/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk735(t4905: f64, t4934: f64, t518: f64, t166: f64, t161: f64, t1639: f64, t2088: f64, t1586: f64, t2093: f64, t2094: f64, t489: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4935 = t4905 + t4934;
    let t4936 = t518 * t4935;
    let t4937 = t166 * t4936;
    let t4939 = t161 * t4937 / 30.0_f64;
    let t4940 = t1639 * t2088;
    let t4941 = t166 * t4940;
    let t4943 = t161 * t4941 / 15.0_f64;
    let t4944 = t2093 * t1586;
    let t4945 = t166 * t4944;
    let t4947 = t161 * t4945 / 30.0_f64;
    let t4948 = t489 * t2094;
    let t4950 = 2.0_f64 / 45.0_f64 * t161 * t4948;
    (t4935, t4936, t4937, t4939, t4940, t4941, t4943, t4944, t4945, t4947, t4948, t4950)
}
