//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 287/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk287(t36: f64, t97: f64, t941: f64, t628: f64, t944: f64, t569: f64, t99: f64) -> (f64, f64, f64, f64) {
    let t949 = 1.0_f64/f64::sqrt(t36);
    let t950 = t949 * t97;
    let t951 = t950 * t941;
    let t953 = t628 * t944;
    let t955 = t99 * t569;
    (t950, t951, t953, t955)
}
