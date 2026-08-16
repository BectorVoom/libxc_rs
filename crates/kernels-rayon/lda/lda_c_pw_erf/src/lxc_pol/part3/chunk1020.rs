//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1020/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1020(t11948: f64, t1511: f64, t184: f64, t1980: f64, t199: f64, t1529: f64, t1960: f64, t9267: f64, t9270: f64, t9273: f64, t11937: f64, t11939: f64, t11941: f64, t11943: f64, t11945: f64, t11947: f64, t9259: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11949 = 4.0_f64 / 3.0_f64 * t11948;
    let t11953 = 4.0_f64 / 5.0_f64 * t1511 * t1980 * t184 * t199;
    let t11954 = t1960 * t1529;
    let t11955 = 4.0_f64 / 45.0_f64 * t11954;
    let t11956 = 16.0_f64 / 45.0_f64 * t9267;
    let t11957 = 8.0_f64 / 45.0_f64 * t9270;
    let t11958 = 16.0_f64 / 45.0_f64 * t9273;
    let t11959 = 4.0_f64 / 3.0_f64 * t9259 + t11937 - t11939 + t11941 - t11943 - t11945 - t11947 - t11949 + t11953 - t11955 + t11956 - t11957 - t11958;
    (t11949, t11953, t11955, t11956, t11957, t11958, t11959)
}
