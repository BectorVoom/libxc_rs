//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 893/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk893(t1098: f64, t9150: f64, t1052: f64, t8141: f64, t1011: f64, t1041: f64, t106: f64, t2341: f64, t2363: f64, t2380: f64, t3138: f64, t3142: f64, t4429: f64, t4438: f64, t4445: f64, t4449: f64, t4451: f64, t7896: f64, t8726: f64, t8821: f64, t9159: f64) -> f64 {
    let t9422 = t1098 * t9150;
    let t9438 = t1052 * t8141;
    let t9440 = t2363 * t3138 / 6.0_f64 + t2363 * t3142 / 6.0_f64 + t9422 / 6.0_f64 + t4429 / 6.0_f64 - t4438 / 6.0_f64 - t4445 / 6.0_f64 - 0.20475546210383508_f64 * t7896 - 0.14975624337724558_f64 * t8726 + t4449 / 9.0_f64 - 0.14975624337724558_f64 * t8821 + t1041 * t2341 / 6.0_f64 - t2380 * t1011 / 6.0_f64 - t106 * t9159 / 6.0_f64 + t4451 / 9.0_f64 - t9438 / 9.0_f64;
    t9440
}
