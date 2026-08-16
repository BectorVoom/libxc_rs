//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 593/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk593(t24: f64, t3926: f64, t645: f64, t1953: f64, t2061: f64, t248: f64, t256: f64, t635: f64, t646: f64, t1432: f64, t639: f64, t1423: f64, t1427: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3927 = t24 * t3926;
    let t3929 = 0.18233333333333332_f64 * t645 * t3927;
    let t3932 = 0.1005925925925926_f64 * t1953 - 0.5007407407407407_f64 * t2061;
    let t3933 = t248 * t3932;
    let t3935 = t3933 * t256 / 3.0_f64;
    let t3936 = t635 * t646;
    let t3938 = 0.013506172839506173_f64 * t645 * t3936;
    let t3949 = t639 * t1432;
    let t3950 = t3949 * t256;
    let t3951 = t1423 * t1427;
    (t3927, t3929, t3932, t3933, t3935, t3936, t3938, t3949, t3950, t3951)
}
