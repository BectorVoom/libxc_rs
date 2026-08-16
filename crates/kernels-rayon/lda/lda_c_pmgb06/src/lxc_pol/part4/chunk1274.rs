//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1274/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1274(t5: f64, t1837: f64, t1848: f64, t5417: f64, t831: f64, t1542: f64, t2592: f64, t1074: f64, t12429: f64, t16322: f64, t2381: f64, t247: f64, t332: f64, t395: f64, t5961: f64, t6695: f64, t760: f64, zeta_threshold: f64) -> (f64, f64, f64, f64) {
    let t6 = t5 <= zeta_threshold;
    let t16755 = 2.0_f64 / 15.0_f64 * t1848 * t1837;
    let t16757 = t831 * t5417 / 15.0_f64;
    let t16759 = t2592 * t1542 / 30.0_f64;
    let t16769 = piecewise3(t6, 0.0_f64, 2.0_f64 * t1074 * t2381 - 24.0_f64 * t247 * t6695 + 4.0_f64 * t332 * t5961 + 8.0_f64 * t395 * t760 + t12429 + t16322);
    (t16755, t16757, t16759, t16769)
}
