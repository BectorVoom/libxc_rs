//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 641/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk641(t403: f64, t5031: f64, t1438: f64, t378: f64, t1369: f64, t4979: f64, t1319: f64, t1368: f64, t1287: f64, t1435: f64, t1549: f64, t356: f64, t4767: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5464 = t403 * t5031;
    let t5470 = 1.0_f64 / t1438 / t378;
    let t5477 = 18.635258017632964_f64 * t1369 * t4979;
    let t5480 = 4.937333717448355_f64 * t1319 * t4979;
    let t5481 = t1368 * t5031;
    let t5482 = t5481 * t1287;
    let t5484 = t1549 * t1435;
    let t5511 = 0.8357942709722364_f64 * t356 * t4767;
    (t5464, t5470, t5477, t5480, t5482, t5484, t5511)
}
