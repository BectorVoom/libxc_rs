//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 714/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk714(t5: f64, t247: f64, t902: f64, t2142: f64, t686: f64, t248: f64, t2158: f64, t643: f64, t3912: f64, t760: f64, t1: f64, t1068: f64, t1069: f64, t1074: f64, t2125: f64, t2128: f64, t395: f64, t4367: f64, t9: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t6 = t5 <= zeta_threshold;
    let t4472 = t247 * t902;
    let t4481 = t2142 * t686;
    let t4483 = 2.0_f64 * t248 * t4481;
    let t4485 = 8.0_f64 * t643 * t2158;
    let t4486 = t3912 * t760;
    let t4489 = t1068 * t1;
    let t4499 = piecewise3(t6, 0.0_f64, -8.0_f64 / 27.0_f64 * t4486 * t1069 + 16.0_f64 / 9.0_f64 * t4489 * t4367 + 4.0_f64 / 9.0_f64 * t2125 * t1074 + 8.0_f64 / 3.0_f64 * t9 * t395 - 8.0_f64 * t2128 * t247);
    (t4472, t4481, t4483, t4485, t4486, t4499)
}
