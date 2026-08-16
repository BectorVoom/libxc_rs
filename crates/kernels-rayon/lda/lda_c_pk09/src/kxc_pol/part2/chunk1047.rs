//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 1047/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk1047(t11092: f64, t1927: f64, t1240: f64, t2913: f64, t454: f64, t1948: f64, t633: f64, t1905: f64, t2016: f64, t2811: f64, t2006: f64, t2860: f64) -> (f64, f64, f64, f64, f64) {
    let t11380 = t1927 * t11092;
    let t11384 = t2913 * t1240;
    let t11385 = t454 * t11384;
    let t11386 = t1948 * t11385;
    let t11388 = t2913 * t633;
    let t11389 = t1905 * t11388;
    let t11393 = t2811 * t2016;
    let t11396 = t2006 * t2860;
    (t11380, t11386, t11389, t11393, t11396)
}
