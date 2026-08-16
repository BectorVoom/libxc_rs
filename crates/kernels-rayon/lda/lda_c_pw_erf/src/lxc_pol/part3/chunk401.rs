//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 401/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk401(t1476: f64, t571: f64, t575: f64, t954: f64, t574: f64, t212: f64, t558: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t1477 = t571 * t1476;
    let t1478 = 16.0_f64 / 135.0_f64 * t1477;
    let t1479 = t575 * t954;
    let t1480 = t574 * t1479;
    let t1482 = 4.0_f64 / 45.0_f64 * t571 * t1480;
    let t1484 = 1.0_f64 / t212 / t558;
    (t1477, t1478, t1479, t1480, t1482, t1484)
}
