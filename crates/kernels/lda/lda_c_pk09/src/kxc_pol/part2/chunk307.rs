//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 307/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk307<F: Float>(t1292: F, t1341: F, t1342: F, t1345: F, t1348: F, t1351: F, t1365: F, t1371: F, t1383: F, t1388: F, t1399: F, t1405: F, t1406: F, t1427: F, t1433: F, t1437: F, t1450: F, t1451: F, t311: F) -> (F,) {
    let t1454 = t1341 - 1.8805371096875316 * t1342 * t1292 - 18.635258017632964 * t1345 * t1292 + 0.04115066352984959 * t1348 * t1351 - 0.04115066352984959 * t1348 * t1365 - t1371 - 2.427516195194328 * t1383 * t311 - 1.7770439370459628 * t1388 * t1399 + t1405 + 2.2140749178833072 * t1406 * t1292 - 2.2140749178833072 * t1427 * t311 + t1433 + t1437 - 2.9824072957409817 * t1450 * t1451;
    (t1454,)
}
