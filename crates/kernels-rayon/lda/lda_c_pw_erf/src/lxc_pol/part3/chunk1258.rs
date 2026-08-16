//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1258/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1258(t5446: f64, t646: f64, t1426: f64, t1901: f64, t11941: f64, t11943: f64, t11945: f64, t11947: f64, t11949: f64, t11953: f64, t11955: f64, t11956: f64, t11957: f64, t11958: f64, t11960: f64) -> f64 {
    let t14978 = t5446 * t646;
    let t14979 = 0.09973633333333333_f64 * t14978;
    let t14980 = t1901 * t1426;
    let t14982 = t11941 - t11943 + t14979 + 0.09973633333333333_f64 * t14980 - t11945 - t11947 - t11949 + t11953 - t11955 + t11956 - t11957 - t11958 + t11960;
    t14982
}
