//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 1045/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk1045(t11351: f64, t337: f64, t430: f64, t11059: f64, t489: f64, t2738: f64, t6247: f64, t6977: f64, t2739: f64, t7473: f64, t2042: f64, t545: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11352 = t11351 * t337;
    let t11353 = t11352 * t430;
    let t11356 = t489 * t11059;
    let t11362 = t6247 * t2738;
    let t11363 = t11362 * t6977;
    let t11366 = t2739 * t7473;
    let t11367 = t11366 * t2042;
    let t11369 = t545 * t11059;
    (t11352, t11353, t11356, t11363, t11367, t11369)
}
