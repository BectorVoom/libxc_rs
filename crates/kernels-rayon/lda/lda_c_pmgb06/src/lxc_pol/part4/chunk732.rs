//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 732/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk732(t5: f64, t153: f64, t4680: f64, t137: f64, t132: f64, t1: f64, t10: f64, t1069: f64, t1074: f64, t1941: f64, t1944: f64, t247: f64, t395: f64, t4367: f64, t594: f64, t761: f64, zeta_threshold: f64) -> (f64, f64, f64, f64) {
    let t6 = t5 <= zeta_threshold;
    let t4681 = t4680 * t153;
    let t4682 = t137 * t4681;
    let t4684 = t132 * t4682 / 30.0_f64;
    let t4687 = t10 * t1;
    let t4697 = piecewise3(t6, 0.0_f64, 80.0_f64 / 27.0_f64 * t761 * t1069 + 160.0_f64 / 9.0_f64 * t4687 * t4367 + 40.0_f64 / 9.0_f64 * t1941 * t1074 + 16.0_f64 / 3.0_f64 * t594 * t395 - 16.0_f64 * t1944 * t247);
    (t4681, t4682, t4684, t4697)
}
