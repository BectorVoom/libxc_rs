//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1137/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1137(t11020: f64, t11022: f64, t11025: f64, t11027: f64, t11029: f64, t19123: f64, t21001: f64, t21002: f64, t21003: f64, t21004: f64, t21005: f64, t21008: f64, t21012: f64) -> f64 {
    let t21013 = t21001 - t21002 - t21003 + t21004 - t21005 + 2.0_f64 / 45.0_f64 * t19123 + t21008 + 0.09973633333333333_f64 * t11020 - 0.06649088888888889_f64 * t11022 - t11025 + t11027 + t11029 - t21012;
    t21013
}
