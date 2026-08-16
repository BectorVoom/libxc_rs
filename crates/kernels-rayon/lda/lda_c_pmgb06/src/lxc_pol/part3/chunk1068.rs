//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1068/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1068(t130: f64, t431: f64, t5076: f64, t5079: f64, t1386: f64, t1593: f64, t2064: f64, t5077: f64, t1414: f64, t1601: f64, t1602: f64, t337: f64, t764: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12683 = t431 * t130;
    let t12684 = t12683 * t5076;
    let t12686 = 4.0_f64 / 15.0_f64 * t12684 * t5079;
    let t12690 = 4.0_f64 / 15.0_f64 * t5077 * t1593 * t2064 * t1386;
    let t12691 = t1601 * t1414;
    let t12693 = t764 * t1602 * t337;
    (t12683, t12684, t12686, t12690, t12691, t12693)
}
