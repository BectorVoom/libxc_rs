//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 1054/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk1054<F: Float>(t1947: F, t2939: F, t2042: F, t1792: F, t2889: F, t11096: F, t11433: F, t11437: F, t11441: F, t11444: F, t11450: F, t11452: F, t11456: F, t11458: F, t11462: F, t11464: F, t11467: F, t11470: F, t11473: F, t11475: F, t1904: F, t1934: F, t2032: F, t2940: F, t455: F, t6864: F, t7346: F, t93: F) -> F {
    let t11478 = t2939 * t1947;
    let t11479 = t11478 * t2042;
    let t11483 = t2889 * t1792;
    let t11487 = -F::cast_from(2.2140749178833072_f64) * t11433 * t455 + F::cast_from(2.427516195194328_f64) * t11437 + F::cast_from(0.04115066352984959_f64) * t1904 * t11441 + F::cast_from(4.937333717448355_f64) * t11444 + F::cast_from(9.87466743489671_f64) * t1934 * t11096 - F::cast_from(0.7380249726277691_f64) * t6864 - F::cast_from(5.40024514194619_f64) * t11450 - F::cast_from(5.40024514194619_f64) * t11452 + F::cast_from(5.40024514194619_f64) * t11456 + F::cast_from(22.07984838129906_f64) * t11458 + F::cast_from(22.07984838129906_f64) * t11462 - F::cast_from(7.35994946043302_f64) * t11464 - F::cast_from(3.600163427964126_f64) * t11467 + F::cast_from(3.600163427964126_f64) * t11470 + F::cast_from(2.427516195194328_f64) * t11473 * t11475 + F::cast_from(2.427516195194328_f64) * t11479 + F::cast_from(2.427516195194328_f64) * t2940 * t2032 + F::cast_from(0.8885219685229814_f64) * t7346 * t93 * t11483;
    t11487
}
