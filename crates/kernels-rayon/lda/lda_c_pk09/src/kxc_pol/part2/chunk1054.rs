//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 1054/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk1054(t1947: f64, t2939: f64, t2042: f64, t1792: f64, t2889: f64, t11096: f64, t11433: f64, t11437: f64, t11441: f64, t11444: f64, t11450: f64, t11452: f64, t11456: f64, t11458: f64, t11462: f64, t11464: f64, t11467: f64, t11470: f64, t11473: f64, t11475: f64, t1904: f64, t1934: f64, t2032: f64, t2940: f64, t455: f64, t6864: f64, t7346: f64, t93: f64) -> f64 {
    let t11478 = t2939 * t1947;
    let t11479 = t11478 * t2042;
    let t11483 = t2889 * t1792;
    let t11487 = -2.2140749178833072_f64 * t11433 * t455 + 2.427516195194328_f64 * t11437 + 0.04115066352984959_f64 * t1904 * t11441 + 4.937333717448355_f64 * t11444 + 9.87466743489671_f64 * t1934 * t11096 - 0.7380249726277691_f64 * t6864 - 5.40024514194619_f64 * t11450 - 5.40024514194619_f64 * t11452 + 5.40024514194619_f64 * t11456 + 22.07984838129906_f64 * t11458 + 22.07984838129906_f64 * t11462 - 7.35994946043302_f64 * t11464 - 3.600163427964126_f64 * t11467 + 3.600163427964126_f64 * t11470 + 2.427516195194328_f64 * t11473 * t11475 + 2.427516195194328_f64 * t11479 + 2.427516195194328_f64 * t2940 * t2032 + 0.8885219685229814_f64 * t7346 * t93 * t11483;
    t11487
}
