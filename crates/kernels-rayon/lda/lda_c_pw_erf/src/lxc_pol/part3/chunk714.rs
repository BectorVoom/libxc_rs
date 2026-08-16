//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 714/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk714(t348: f64, t4495: f64, t4494: f64, t4488: f64, t1458: f64, t529: f64, t1245: f64) -> (f64, f64, f64, f64, f64) {
    let t4496 = t4495 * t348;
    let t4497 = t4494 * t4496;
    let t4499 = 16.0_f64 / 45.0_f64 * t4488 * t4497;
    let t4500 = t1458 * t529;
    let t4501 = t4500 * t1245;
    (t4496, t4497, t4499, t4500, t4501)
}
