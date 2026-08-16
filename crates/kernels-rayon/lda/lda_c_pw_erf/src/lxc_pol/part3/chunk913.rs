//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 913/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk913(t1404: f64, t1518: f64, t211: f64, t172: f64, t184: f64, t4008: f64, t1234: f64, t1513: f64, t202: f64, t4024: f64, t3465: f64, t493: f64, t514: f64) -> (f64, f64, f64, f64, f64) {
    let t9596 = t211 * t1518 * t1404;
    let t9599 = t172 * t4008 * t184;
    let t9602 = t1513 * t1234;
    let t9615 = t202 * t4024 * t184;
    let t9619 = t493 * t514 * t3465;
    (t9596, t9599, t9602, t9615, t9619)
}
