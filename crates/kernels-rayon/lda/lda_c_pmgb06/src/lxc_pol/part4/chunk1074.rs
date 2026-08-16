//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1074/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1074(t441: f64, t4680: f64, t1447: f64, t4762: f64, t1423: f64, t5198: f64, t1435: f64, t1872: f64, t1517: f64, t1887: f64, t3076: f64, t802: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12063 = t441 * t4680;
    let t12075 = t1447 * t4762;
    let t12084 = t1423 * t5198;
    let t12092 = t1435 * t1872;
    let t12105 = t1887 * t1517;
    let t12107 = t802 * t3076;
    (t12063, t12075, t12084, t12092, t12105, t12107)
}
