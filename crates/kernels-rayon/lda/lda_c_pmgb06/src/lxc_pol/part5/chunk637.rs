//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 637/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk637(t500: f64, t5194: f64, t136: f64, t458: f64, t3220: f64, t806: f64, t1423: f64, t2007: f64, t1179: f64, t131: f64) -> (f64, f64, f64, f64, f64) {
    let t5196 = 4.0_f64 / 135.0_f64 * t5194 * t500;
    let t5197 = t136 * t458;
    let t5207 = 4.0_f64 / 135.0_f64 * t3220 * t806;
    let t5209 = 4.0_f64 / 135.0_f64 * t1423 * t2007;
    let t5210 = t131 * t1179;
    (t5196, t5197, t5207, t5209, t5210)
}
