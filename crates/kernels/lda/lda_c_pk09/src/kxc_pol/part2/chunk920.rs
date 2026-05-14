//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 920/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk920<F: Float>(t11092: F, t1927: F, t1240: F, t2913: F, t454: F, t1948: F, t633: F, t1905: F, t2016: F, t2811: F, t2006: F, t2860: F, t2777: F, t6945: F, t452: F, t1941: F, t309: F) -> (F, F, F, F, F, F, F) {
    let t11380 = t1927 * t11092;
    let t11384 = t2913 * t1240;
    let t11385 = t454 * t11384;
    let t11386 = t1948 * t11385;
    let t11388 = t2913 * t633;
    let t11389 = t1905 * t11388;
    let t11393 = t2811 * t2016;
    let t11396 = t2006 * t2860;
    let t11400 = t2777 * t6945;
    let t11401 = t11400 * t452;
    let t11403 = t309 * t454 * t1941;
    (t11380, t11386, t11389, t11393, t11396, t11401, t11403)
}
