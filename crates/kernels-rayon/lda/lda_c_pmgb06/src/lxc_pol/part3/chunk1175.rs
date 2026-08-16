//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1175/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1175(t14019: f64, t13995: f64, t13997: f64, t13999: f64, t14002: f64, t14005: f64, t14007: f64, t14010: f64, t14012: f64, t14014: f64, t14016: f64, t14018: f64) -> (f64, f64) {
    let t14020 = 2.0_f64 / 15.0_f64 * t14019;
    let t14021 = -t13995 - t13997 - t13999 - t14002 - t14005 - t14007 - t14010 - t14012 - t14014 + t14016 - t14018 - t14020;
    (t14020, t14021)
}
