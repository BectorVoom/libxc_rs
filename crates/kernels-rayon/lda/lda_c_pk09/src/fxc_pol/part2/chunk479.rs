//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 479/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk479(t2666: f64, t306: f64, t1290: f64, t1307: f64, t1342: f64, t1345: f64, t1348: f64, t1406: f64, t1433: f64, t1437: f64, t1457: f64, t1460: f64, t1495: f64, t2513: f64, t2594: f64, t2637: f64, t2641: f64, t2650: f64, t311: f64) -> (f64, f64) {
    let t2667 = t2666 * t306;
    let t2673 = 4.937333717448355_f64 * t2637 * t311 + 0.04115066352984959_f64 * t1348 * t2641 - 0.04115066352984959_f64 * t1348 * t2650 - 18.635258017632964_f64 * t1345 * t2513 + 2.2140749178833072_f64 * t1406 * t2513 + 3.7610742193750633_f64 * t1307 * t2513 - 1.8805371096875316_f64 * t1342 * t2513 + 1.8805371096875316_f64 * t2667 * t311 - 19.489173774580152_f64 * t1290 * t2513 - t1495 * t2594 + t1433 + t1437 - t1457 + t1460;
    (t2667, t2673)
}
