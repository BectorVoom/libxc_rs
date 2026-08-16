//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 1049/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk1049(t11412: f64, t1948: f64, t2923: f64, t633: f64, t6938: f64, t1920: f64, t2149: f64, t1905: f64, t2870: f64, t6972: f64, t452: f64, t1870: f64, t309: f64, t454: f64) -> (f64, f64, f64, f64, f64) {
    let t11413 = t1948 * t11412;
    let t11415 = t2923 * t633;
    let t11416 = t6938 * t11415;
    let t11419 = t1920 * t2149;
    let t11420 = t1905 * t11419;
    let t11423 = t2870 * t6972;
    let t11424 = t11423 * t452;
    let t11426 = t309 * t454 * t1870;
    (t11413, t11416, t11420, t11424, t11426)
}
