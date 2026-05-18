//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 479/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk479<F: Float>(t2666: F, t306: F, t1290: F, t1307: F, t1342: F, t1345: F, t1348: F, t1406: F, t1433: F, t1437: F, t1457: F, t1460: F, t1495: F, t2513: F, t2594: F, t2637: F, t2641: F, t2650: F, t311: F) -> (F, F) {
    let t2667 = t2666 * t306;
    let t2673 = F::new(4.937333717448355) * t2637 * t311 + F::new(0.04115066352984959) * t1348 * t2641 - F::new(0.04115066352984959) * t1348 * t2650 - F::new(18.635258017632964) * t1345 * t2513 + F::new(2.2140749178833072) * t1406 * t2513 + F::new(3.7610742193750633) * t1307 * t2513 - F::new(1.8805371096875316) * t1342 * t2513 + F::new(1.8805371096875316) * t2667 * t311 - F::new(19.489173774580152) * t1290 * t2513 - t1495 * t2594 + t1433 + t1437 - t1457 + t1460;
    (t2667, t2673)
}
