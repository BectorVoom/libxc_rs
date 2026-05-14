//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 448/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk448<F: Float>(t1244: F, t1256: F, t1264: F, t1273: F, t2502: F, t2505: F, t2542: F, t2546: F, t1278: F, t306: F, t1290: F, t1307: F, t1342: F, t1345: F, t1348: F, t1406: F, t1433: F, t1437: F, t1457: F, t1460: F, t1495: F, t2513: F, t2594: F, t2637: F, t2641: F, t2650: F, t311: F) -> (F, F, F, F) {
    let t2665 = t1244 - 3.2084841915276807 * t2542 + t1256 + 3.2084841915276807 * t2546 + t1264 - 0.64 * t2502 + t1273 + 0.64 * t2505;
    let t2666 = t2665 * t1278;
    let t2667 = t2666 * t306;
    let t2673 = 4.937333717448355 * t2637 * t311 + 0.04115066352984959 * t1348 * t2641 - 0.04115066352984959 * t1348 * t2650 - 18.635258017632964 * t1345 * t2513 + 2.2140749178833072 * t1406 * t2513 + 3.7610742193750633 * t1307 * t2513 - 1.8805371096875316 * t1342 * t2513 + 1.8805371096875316 * t2667 * t311 - 19.489173774580152 * t1290 * t2513 - t1495 * t2594 + t1433 + t1437 - t1457 + t1460;
    (t2665, t2666, t2667, t2673)
}
