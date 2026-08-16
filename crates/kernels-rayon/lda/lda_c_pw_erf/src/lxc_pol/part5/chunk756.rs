//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 756/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk756(t1440: f64, t6916: f64, t575: f64, t6005: f64, t574: f64, t1325: f64, t1446: f64, t1472: f64, t2146: f64, t2153: f64, t2171: f64, t2178: f64, t2540: f64, t2544: f64, t2550: f64, t2558: f64, t2562: f64, t2566: f64, t3794: f64, t4804: f64, t519: f64, t5312: f64, t5327: f64, t571: f64, t6895: f64, t6897: f64, t6905: f64, t6909: f64, t799: f64) -> (f64, f64, f64, f64) {
    let t6917 = t1440 * t6916;
    let t6924 = t575 * t6005;
    let t6925 = t574 * t6924;
    let t6936 = t5312 - 16.0_f64 / 45.0_f64 * t6895 + 16.0_f64 / 135.0_f64 * t6897 - 8.0_f64 / 45.0_f64 * t1472 * t2562 - 8.0_f64 / 45.0_f64 * t1446 * t2566 - 4.0_f64 / 5.0_f64 * t519 * t6905 + 8.0_f64 / 15.0_f64 * t519 * t6909 - 8.0_f64 / 15.0_f64 * t4804 * t2558 - 8.0_f64 / 15.0_f64 * t3794 * t2558 - 8.0_f64 / 15.0_f64 * t1325 * t6917 - 16.0_f64 / 45.0_f64 * t2146 * t2153 + 4.0_f64 / 45.0_f64 * t1472 * t2540 + 4.0_f64 / 45.0_f64 * t571 * t6925 + 4.0_f64 / 27.0_f64 * t1472 * t2544 + 8.0_f64 / 45.0_f64 * t5327 * t799 + 16.0_f64 / 45.0_f64 * t2171 * t2178 + 4.0_f64 / 45.0_f64 * t1446 * t2550;
    (t6917, t6924, t6925, t6936)
}
