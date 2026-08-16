//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1088/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1088(t131: f64, t155: f64, t20100: f64, t44: f64, t12650: f64, t20071: f64, t20074: f64, t20076: f64, t20079: f64, t20081: f64, t20084: f64, t20086: f64, t20089: f64, t20090: f64) -> (f64, f64) {
    let t20104 = t20100 * t44 * t131 * t155 / 30.0_f64;
    let t20105 = t20071 + t20074 + t20076 + t20079 + t20081 - t12650 + t20084 + t20086 + t20089 - t20090 + t20104;
    (t20104, t20105)
}
