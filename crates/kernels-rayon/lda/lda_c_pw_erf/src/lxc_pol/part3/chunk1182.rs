//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1182/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1182(t13924: f64, t1298: f64, t4568: f64, t2162: f64, t571: f64, t9432: f64, t3899: f64, t5374: f64, t1466: f64, t2161: f64, t3655: f64, t10030: f64, t5167: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13925 = 8.0_f64 / 45.0_f64 * t13924;
    let t13926 = t1298 * t4568;
    let t13927 = 4.0_f64 / 3.0_f64 * t13926;
    let t13929 = t571 * t9432 * t2162;
    let t13930 = 8.0_f64 / 45.0_f64 * t13929;
    let t13932 = t571 * t3899 * t5374;
    let t13933 = 8.0_f64 / 15.0_f64 * t13932;
    let t13937 = 4.0_f64 / 15.0_f64 * t571 * t1466 * t2161 * t3655;
    let t13938 = t10030 * t5167;
    (t13925, t13927, t13930, t13933, t13937, t13938)
}
