//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 581/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk581(t2000: f64, t2016: f64, t2019: f64, t1464: f64, t2386: f64) -> (f64, f64, f64, f64) {
    let t2534 = 4.0_f64 / 135.0_f64 * t2000;
    let t2535 = 2.0_f64 / 45.0_f64 * t2016;
    let t2536 = 2.0_f64 / 45.0_f64 * t2019;
    let t2541 = t1464 * t2386;
    (t2534, t2535, t2536, t2541)
}
