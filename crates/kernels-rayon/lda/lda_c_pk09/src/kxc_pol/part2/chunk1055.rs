//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 1055/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk1055(t2149: f64, t4007: f64, t633: f64, t93: f64, t11092: f64, t1853: f64, t1672: f64, t2847: f64, t7102: f64, t451: f64, t476: f64, t2795: f64, t7308: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11489 = t4007 * t2149;
    let t11490 = t11489 * t633;
    let t11491 = t93 * t11490;
    let t11494 = t1853 * t11092;
    let t11500 = t2847 * t1672;
    let t11502 = t7102 * t11092;
    let t11504 = t451 * t476;
    let t11509 = t2795 * t7308;
    (t11491, t11494, t11500, t11502, t11504, t11509)
}
