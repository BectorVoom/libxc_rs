//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 409/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk409(t5: f64, t161: f64, t1933: f64, t490: f64, t831: f64, t1393: f64, t607: f64, t883: f64, t10: f64, t760: f64, t1: f64, t594: f64, t332: f64, t395: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6 = t5 <= zeta_threshold;
    let t1934 = t161 * t1933;
    let t1935 = t1934 / 45.0_f64;
    let t1936 = t831 * t490;
    let t1937 = t1936 / 45.0_f64;
    let t1938 = t1393 / 45.0_f64;
    let t1939 = t883 * t607;
    let t1941 = t10 * t760;
    let t1944 = t594 * t1;
    let t1948 = piecewise3(t6, 0.0_f64, 40.0_f64 / 9.0_f64 * t1941 * t332 + 16.0_f64 / 3.0_f64 * t1944 * t395);
    (t1934, t1935, t1936, t1937, t1938, t1939, t1941, t1948)
}
