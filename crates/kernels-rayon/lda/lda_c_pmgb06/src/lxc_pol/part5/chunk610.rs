//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 610/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk610(t395: f64, t2799: f64, t247: f64, t902: f64, t2142: f64, t686: f64, t248: f64, t2158: f64, t643: f64, t3912: f64, t760: f64, t1: f64, t1068: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4461 = 4.0_f64 * t395;
    let t4462 = 12.0_f64 * t2799;
    let t4472 = t247 * t902;
    let t4481 = t2142 * t686;
    let t4483 = 2.0_f64 * t248 * t4481;
    let t4485 = 8.0_f64 * t643 * t2158;
    let t4486 = t3912 * t760;
    let t4489 = t1068 * t1;
    (t4461, t4462, t4472, t4481, t4483, t4485, t4486, t4489)
}
