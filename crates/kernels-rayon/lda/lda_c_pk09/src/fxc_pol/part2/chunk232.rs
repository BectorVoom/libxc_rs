//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 232/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk232(t609: f64, t62: f64, t891: f64, t917: f64, t61: f64, t96: f64, t839: f64, t125: f64, t861: f64, t204: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t918 = t62 * t609;
    let t919 = t891 * t918;
    let t920 = t917 * t919;
    let t921 = 1.800081713982063_f64 * t920;
    let t922 = t61 * t609;
    let t923 = t96 * t922;
    let t924 = t839 * t923;
    let t925 = 22.07984838129906_f64 * t924;
    let t932 = t861 * t125;
    let t933 = t932 * t204;
    (t919, t920, t921, t923, t924, t925, t932, t933)
}
