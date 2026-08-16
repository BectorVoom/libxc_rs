//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 603/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk603(t534: f64, t97: f64, t1377: f64, t1410: f64, t27: f64, t545: f64, t540: f64, t1366: f64, t1369: f64, t1372: f64, t186: f64, t315: f64, t934: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3319 = t534 * t97;
    let t3320 = t3319 * t1377;
    let t3322 = t1410 * t27;
    let t3324 = 0.3246312408709453_f64 * t3322 * t545;
    let t3325 = t540 * t97;
    let t3327 = 0.03354522822333102_f64 * t3325 * t1377;
    let t3328 = t1369 * t1366;
    let t3331 = 0.21642082724729686_f64 * t1372 * t1366;
    let t3333 = t934 * t315 * t186;
    (t3319, t3320, t3322, t3324, t3325, t3327, t3328, t3331, t3333)
}
