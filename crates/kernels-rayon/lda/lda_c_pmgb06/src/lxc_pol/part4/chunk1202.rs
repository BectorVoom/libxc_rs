//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1202/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1202(t1462: f64, t1465: f64, t15845: f64, t79: f64, t486: f64, t6843: f64, t130: f64, t801: f64, t5076: f64, t5095: f64, t5082: f64, t5087: f64) -> (f64, f64, f64, f64, f64) {
    let t15849 = 8.0_f64 / 27.0_f64 * t15845 * t1462 * t1465 * t79;
    let t15850 = t486 * t6843;
    let t15851 = 2.0_f64 / 45.0_f64 * t15850;
    let t15854 = t801 * t130;
    let t15855 = t15854 * t5076;
    let t15857 = 8.0_f64 / 45.0_f64 * t15855 * t5095;
    let t15858 = t15854 * t5082;
    let t15860 = 4.0_f64 / 27.0_f64 * t15858 * t5087;
    (t15849, t15851, t15855, t15857, t15860)
}
