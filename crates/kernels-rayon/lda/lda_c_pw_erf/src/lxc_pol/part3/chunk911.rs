//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 911/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk911(t1318: f64, t3899: f64, t3904: f64, t3756: f64, t571: f64, t155: f64, t213: f64, t1468: f64, t2151: f64, t576: f64, t352: f64, t954: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9427 = t1318 * t3899 * t3904;
    let t9430 = t571 * t3899 * t3756;
    let t9432 = t155 * t213;
    let t9434 = t1318 * t9432 * t1468;
    let t9436 = t2151 * t576;
    let t9437 = t571 * t9436;
    let t9456 = t954 * t352;
    (t9427, t9430, t9432, t9434, t9436, t9437, t9456)
}
