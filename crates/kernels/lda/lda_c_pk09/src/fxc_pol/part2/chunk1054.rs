//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 1054/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk1054<F: Float>(t1947: F, t2939: F, t2042: F, t1792: F, t2889: F, t11096: F, t11433: F, t11437: F, t11441: F, t11444: F, t11450: F, t11452: F, t11456: F, t11458: F, t11462: F, t11464: F, t11467: F, t11470: F, t11473: F, t11475: F, t1904: F, t1934: F, t2032: F, t2940: F, t455: F, t6864: F, t7346: F, t93: F) -> F {
    let t11478 = t2939 * t1947;
    let t11479 = t11478 * t2042;
    let t11483 = t2889 * t1792;
    let t11487 = -F::new(2.2140749178833072) * t11433 * t455 + F::new(2.427516195194328) * t11437 + F::new(0.04115066352984959) * t1904 * t11441 + F::new(4.937333717448355) * t11444 + F::new(9.87466743489671) * t1934 * t11096 - F::new(0.7380249726277691) * t6864 - F::new(5.40024514194619) * t11450 - F::new(5.40024514194619) * t11452 + F::new(5.40024514194619) * t11456 + F::new(22.07984838129906) * t11458 + F::new(22.07984838129906) * t11462 - F::new(7.35994946043302) * t11464 - F::new(3.600163427964126) * t11467 + F::new(3.600163427964126) * t11470 + F::new(2.427516195194328) * t11473 * t11475 + F::new(2.427516195194328) * t11479 + F::new(2.427516195194328) * t2940 * t2032 + F::new(0.8885219685229814) * t7346 * t93 * t11483;
    t11487
}
