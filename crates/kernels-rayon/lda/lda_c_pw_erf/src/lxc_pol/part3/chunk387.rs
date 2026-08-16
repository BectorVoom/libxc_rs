//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 387/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk387(t1112: f64, t247: f64, t251: f64, t639: f64, t652: f64, t256: f64, t19: f64, t465: f64, t644: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t1415 = t1112 * t247;
    let t1416 = t1415 * t251;
    let t1419 = t639 * t652;
    let t1420 = t1419 * t256;
    let t1422 = t465 * t19;
    let t1423 = t1422 * t644;
    (t1415, t1416, t1419, t1420, t1422, t1423)
}
