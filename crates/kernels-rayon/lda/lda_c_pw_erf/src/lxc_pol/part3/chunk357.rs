//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 357/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk357(t1289: f64, t548: f64, t565: f64, t595: f64, t514: f64, t594: f64) -> (f64, f64, f64) {
    let t1291 = 4.0_f64 / 15.0_f64 * t548 * t1289;
    let t1293 = 4.0_f64 / 15.0_f64 * t565 * t595;
    let t1294 = t514 * t594;
    (t1291, t1293, t1294)
}
