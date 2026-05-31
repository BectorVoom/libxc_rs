//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 465/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk465<F: Float>(t2488: F, t334: F, t1440: F, t1442: F, t2502: F, t2505: F, t1439: F, t1449: F, t2474: F, t49: F, t285: F, t1248: F) -> (F, F, F, F, F, F) {
    let t2525 = t2488 * t334;
    let t2529 = t1440 - F::cast_from(1.5625_f64) * t2502 + t1442 + F::cast_from(1.5625_f64) * t2505;
    let t2530 = t1439 * t2529;
    let t2531 = t2530 * t1449;
    let t2540 = t49 * t2474;
    let t2541 = t285 * t2540;
    let t2542 = t1248 * t2541;
    (t2525, t2529, t2530, t2531, t2540, t2542)
}
