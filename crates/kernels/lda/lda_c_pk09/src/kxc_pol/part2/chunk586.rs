//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 586/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk586<F: Float>(t1369: F, t4979: F, t1319: F, t1368: F, t5031: F, t1287: F, t1435: F, t1549: F, t356: F, t4767: F, t5039: F, t5161: F, t5045: F, t5190: F, t5208: F, t5212: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t5477 = 18.635258017632964 * t1369 * t4979;
    let t5480 = 4.937333717448355 * t1319 * t4979;
    let t5481 = t1368 * t5031;
    let t5482 = t5481 * t1287;
    let t5484 = t1549 * t1435;
    let t5511 = 0.8357942709722364 * t356 * t4767;
    let t5516 = 0.04525483399593904 * t5039;
    let t5520 = 0.30249879055454143 * t5161;
    let t5529 = 0.03016988933062603 * t5045;
    let t5530 = 0.025208232546211785 * t5190;
    let t5535 = 0.22687409291590604 * t5208;
    let t5536 = 0.22687409291590604 * t5212;
    (t5477, t5480, t5482, t5484, t5511, t5516, t5520, t5529, t5530, t5535, t5536)
}
