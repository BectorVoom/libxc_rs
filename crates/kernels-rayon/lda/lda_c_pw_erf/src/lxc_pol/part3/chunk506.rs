//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 506/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk506(t2114: f64, t786: f64, t1298: f64, t172: f64, t793: f64, t184: f64) -> (f64, f64, f64, f64) {
    let t2116 = 4.0_f64 / 15.0_f64 * t2114 * t786;
    let t2118 = 4.0_f64 / 15.0_f64 * t1298 * t786;
    let t2119 = t172 * t793;
    let t2120 = t2119 * t184;
    (t2116, t2118, t2119, t2120)
}
