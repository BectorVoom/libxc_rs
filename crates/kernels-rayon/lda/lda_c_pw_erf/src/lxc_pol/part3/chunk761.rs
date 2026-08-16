//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 761/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk761(t1390: f64, t1392: f64, t784: f64, t1440: f64, t1325: f64, t188: f64, t473: f64, t34: f64, t529: f64, t542: f64, t2067: f64, t565: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4952 = t1390 * t784 * t1392;
    let t4953 = t1440 * t4952;
    let t4955 = 8.0_f64 / 15.0_f64 * t1325 * t4953;
    let t4956 = t473 * t188;
    let t4957 = t529 * t34;
    let t4958 = t4957 * t542;
    let t4959 = t4956 * t4958;
    let t4961 = 8.0_f64 / 15.0_f64 * t1325 * t4959;
    let t4963 = 4.0_f64 / 15.0_f64 * t565 * t2067;
    (t4952, t4953, t4955, t4956, t4957, t4958, t4959, t4961, t4963)
}
