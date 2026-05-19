//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 641/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk641<F: Float>(t403: F, t5031: F, t1438: F, t378: F, t1369: F, t4979: F, t1319: F, t1368: F, t1287: F, t1435: F, t1549: F, t356: F, t4767: F) -> (F, F, F, F, F, F, F) {
    let t5464 = t403 * t5031;
    let t5470 = F::new(1.0) / t1438 / t378;
    let t5477 = F::cast_from(18.635258017632964_f64) * t1369 * t4979;
    let t5480 = F::cast_from(4.937333717448355_f64) * t1319 * t4979;
    let t5481 = t1368 * t5031;
    let t5482 = t5481 * t1287;
    let t5484 = t1549 * t1435;
    let t5511 = F::cast_from(0.8357942709722364_f64) * t356 * t4767;
    (t5464, t5470, t5477, t5480, t5482, t5484, t5511)
}
