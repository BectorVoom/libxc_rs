//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 885/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk885(t5: f64, t6321: f64, t1447: f64, t2466: f64, t2470: f64, t2527: f64, t591: f64, t2377: f64, t330: f64, t10: f64, t2381: f64, t1072: f64, t1941: f64, t332: f64, t594: f64, t5961: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t6 = t5 <= zeta_threshold;
    let t6322 = 4.0_f64 / 135.0_f64 * t6321;
    let t6323 = t1447 * t2466;
    let t6324 = 2.0_f64 / 135.0_f64 * t6323;
    let t6325 = t1447 * t2470;
    let t6326 = 2.0_f64 / 81.0_f64 * t6325;
    let t6327 = t2527 * t591;
    let t6329 = t330 * t2377;
    let t6334 = t10 * t2381;
    let t6340 = piecewise3(t6, 0.0_f64, 80.0_f64 / 27.0_f64 * t6329 * t332 + 160.0_f64 / 9.0_f64 * t1941 * t1072 + 40.0_f64 / 9.0_f64 * t6334 * t332 + 8.0_f64 / 3.0_f64 * t594 * t5961);
    (t6322, t6324, t6326, t6327, t6329, t6334, t6340)
}
