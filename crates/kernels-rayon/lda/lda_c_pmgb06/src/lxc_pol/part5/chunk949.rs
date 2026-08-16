//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 949/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk949(t14310: f64, t1366: f64, t5655: f64, t2349: f64, t3309: f64, t3333: f64, t5649: f64, t1377: f64, t2342: f64, t97: f64, t2345: f64, t27: f64, t545: f64, t5638: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t14311 = 0.21642082724729686_f64 * t14310;
    let t14312 = t5655 * t1366;
    let t14314 = t2349 * t3309;
    let t14316 = t5649 * t3333;
    let t14347 = t2342 * t97 * t1377;
    let t14348 = 0.03354522822333102_f64 * t14347;
    let t14350 = t2345 * t97 * t1377;
    let t14356 = t5638 * t27 * t545;
    (t14311, t14312, t14314, t14316, t14348, t14350, t14356)
}
