//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1476/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1476(t2753: f64, t754: f64, t936: f64, t97: f64, t1786: f64, t1789: f64, t409: f64, t10976: f64, t10993: f64, t14758: f64, t14761: f64, t14765: f64, t8032: f64, t8034: f64, t8039: f64, t8043: f64, t8047: f64) -> (f64, f64) {
    let t19055 = t2753 * t754 * t97 * t936;
    let t19063 = t409 * t2753 * t1786 * t1789;
    let t19069 = 2.0_f64 * t10993 - 0.4564036537785185_f64 * t14758 + 0.6327242966164848_f64 * t19063 + t10976 + 0.9480012043054112_f64 * t14761 + t8047 - t8039 + 0.8215265768013333_f64 * t14765 - 2.530897186465939_f64 * t8032 - 0.4564036537785185_f64 * t8034 + t8043;
    (t19055, t19069)
}
