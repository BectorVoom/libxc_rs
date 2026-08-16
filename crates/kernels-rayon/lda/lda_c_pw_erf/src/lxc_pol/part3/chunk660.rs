//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 660/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk660(t348: f64, t542: f64, t3967: f64, t494: f64, t3965: f64, t108: f64, t547: f64, t267: f64) -> (f64, f64, f64, f64) {
    let t3968 = t348 * t542;
    let t3970 = t3967 * t3968 * t494;
    let t3972 = 16.0_f64 / 15.0_f64 * t3965 * t3970;
    let t3973 = t547 * t108;
    let t3974 = t3973 * t267;
    (t3970, t3972, t3973, t3974)
}
