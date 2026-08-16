//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1448/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1448(t5: f64, t11228: f64, t1234: f64, t2715: f64, t38: f64, t2712: f64, t1069: f64, t1074: f64, t1100: f64, t11259: f64, t1212: f64, t2192: f64, t2377: f64, t2381: f64, t2799: f64, t332: f64, t3537: f64, t395: f64, t4363: f64, t4745: f64, t5953: f64, t5958: f64, t5961: f64, t79: f64, t8119: f64, zeta_threshold: f64) -> (f64, f64, f64, f64) {
    let t6 = t5 <= zeta_threshold;
    let t18503 = 3.8973666666666666_f64 * t11228;
    let t18507 = 17.53815_f64 * t38 * t2715 * t1234;
    let t18518 = 70.1526_f64 * t38 * t2712 * t1234;
    let t18542 = piecewise3(t6, 0.0_f64, -56.0_f64 / 81.0_f64 * t8119 * t2377 * t1069 + 64.0_f64 / 27.0_f64 * t4363 * t4745 + 8.0_f64 / 27.0_f64 * t5953 * t1074 - 16.0_f64 / 9.0_f64 * t1212 * t79 * t1100 - 8.0_f64 / 9.0_f64 * t2192 * t395 + 8.0_f64 / 3.0_f64 * t2192 * t2799 + 8.0_f64 / 27.0_f64 * t3537 * t2381 * t1069 - 4.0_f64 / 9.0_f64 * t1212 * t5961 * t332 - 2.0_f64 / 9.0_f64 * t5958 * t1074 + t11259);
    (t18503, t18507, t18518, t18542)
}
