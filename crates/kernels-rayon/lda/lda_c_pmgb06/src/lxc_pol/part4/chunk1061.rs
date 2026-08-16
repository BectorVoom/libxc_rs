//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1061/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1061(t2209: f64, t374: f64, t4232: f64, t1773: f64, t2262: f64, t2266: f64, t26: f64, t4405: f64, t4359: f64, t5866: f64, t5870: f64, t297: f64, t301: f64, t413: f64, t4463: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11564 = t4232 * t2209 * t374;
    let t11567 = t1773 * t2262;
    let t11569 = t1773 * t2266;
    let t11574 = t4405 * t26;
    let t11583 = t4359 * t5866;
    let t11586 = t4359 * t5870;
    let t11596 = t297 * t4463 * t413 * t301;
    (t11564, t11567, t11569, t11574, t11583, t11586, t11596)
}
