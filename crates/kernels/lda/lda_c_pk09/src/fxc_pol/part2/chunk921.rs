//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 921/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk921<F: Float>(t1947: F, t2919: F, t2042: F, t1240: F, t2923: F, t1905: F, t1948: F, t633: F, t6938: F, t1920: F, t2149: F, t2870: F, t6972: F, t452: F, t1870: F, t309: F, t454: F) -> (F, F, F, F, F, F) {
    let t11406 = t2919 * t1947;
    let t11407 = t11406 * t2042;
    let t11411 = t2923 * t1240;
    let t11412 = t1905 * t11411;
    let t11413 = t1948 * t11412;
    let t11415 = t2923 * t633;
    let t11416 = t6938 * t11415;
    let t11419 = t1920 * t2149;
    let t11420 = t1905 * t11419;
    let t11423 = t2870 * t6972;
    let t11424 = t11423 * t452;
    let t11426 = t309 * t454 * t1870;
    (t11407, t11413, t11416, t11420, t11424, t11426)
}
