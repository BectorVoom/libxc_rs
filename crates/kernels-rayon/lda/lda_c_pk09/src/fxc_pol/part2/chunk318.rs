//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 318/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk318(t1292: f64, t1341: f64, t1342: f64, t1345: f64, t1348: f64, t1351: f64, t1365: f64, t1371: f64, t1383: f64, t1388: f64, t1399: f64, t1405: f64, t1406: f64, t1427: f64, t1433: f64, t1437: f64, t1450: f64, t1451: f64, t311: f64) -> f64 {
    let t1454 = t1341 - 1.8805371096875316_f64 * t1342 * t1292 - 18.635258017632964_f64 * t1345 * t1292 + 0.04115066352984959_f64 * t1348 * t1351 - 0.04115066352984959_f64 * t1348 * t1365 - t1371 - 2.427516195194328_f64 * t1383 * t311 - 1.7770439370459628_f64 * t1388 * t1399 + t1405 + 2.2140749178833072_f64 * t1406 * t1292 - 2.2140749178833072_f64 * t1427 * t311 + t1433 + t1437 - 2.9824072957409817_f64 * t1450 * t1451;
    t1454
}
