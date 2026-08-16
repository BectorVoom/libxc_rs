//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 1052/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk1052(t11449: f64, t2744: f64, t11287: f64, t1995: f64, t7704: f64, t902: f64, t93: f64, t481: f64, t1972: f64, t2752: f64, t132: f64, t333: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11450 = t11449 * t2744;
    let t11452 = t1995 * t11287;
    let t11454 = t902 * t7704;
    let t11455 = t93 * t11454;
    let t11456 = t481 * t11455;
    let t11458 = t1972 * t2752;
    let t11460 = t132 * t7704;
    let t11461 = t333 * t11460;
    (t11450, t11452, t11455, t11456, t11458, t11461)
}
