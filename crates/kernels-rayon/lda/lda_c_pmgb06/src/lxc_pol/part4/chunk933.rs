//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 933/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk933(t3: f64, t6716: f64, t4188: f64, t4191: f64, t4193: f64, t4196: f64, t4199: f64, t4202: f64, t4205: f64, t4208: f64, t4433: f64, t4434: f64) -> (f64, f64) {
    let t6928 = t3 * t6716;
    let t6939 = t4188 + t4191 - t4193 + t4196 + t4199 - t4202 + t4205 + t4433 - t4434 - t4208;
    (t6928, t6939)
}
