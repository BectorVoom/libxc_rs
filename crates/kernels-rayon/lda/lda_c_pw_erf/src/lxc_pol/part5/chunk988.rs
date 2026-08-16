//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 988/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk988(t14932: f64, t153: f64, t2869: f64, t865: f64, t1210: f64, t168: f64, t2292: f64, t1896: f64, t632: f64, t5446: f64, t1143: f64, t1901: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t14933 = 3.9861630686838536_f64 * t14932;
    let t14935 = t153 * t2869 * t865;
    let t14941 = t168 * t1210 * t2292;
    let t14942 = 0.15917832887339686_f64 * t14941;
    let t14943 = t1896 * t632;
    let t14947 = t5446 * t632;
    let t14948 = 0.5025769232130264_f64 * t14947;
    let t14950 = t1901 * t1143;
    (t14933, t14935, t14942, t14943, t14948, t14950)
}
