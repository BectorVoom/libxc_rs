//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 988/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk988(t405: f64, t6891: f64, t4913: f64, t6894: f64, t6897: f64, t6900: f64, t13483: f64, t176: f64, t1447: f64, t6756: f64, t6761: f64, t6766: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t17215 = t405 * t6891;
    let t17217 = t4913 * t6894;
    let t17222 = t405 * t6897;
    let t17224 = t405 * t6900;
    let t17276 = t13483 * t176;
    let t17283 = t1447 * t6756;
    let t17285 = t1447 * t6761;
    let t17287 = t1447 * t6766;
    (t17215, t17217, t17222, t17224, t17276, t17283, t17285, t17287)
}
