//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 979/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk979(t1234: f64, t409: f64, t55: f64, t3600: f64, t1227: f64, t1243: f64, t3594: f64, t1263: f64, t410: f64, t360: f64, t1271: f64, t1282: f64, t8299: f64, t97: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8369 = t55 * t409 * t1234;
    let t8370 = t3600 * t8369;
    let t8373 = t55 * t409 * t1227;
    let t8374 = t1243 * t8373;
    let t8376 = t3594 * t8369;
    let t8381 = t410 * t1263;
    let t8382 = t360 * t8381;
    let t8386 = t1271 * t1282 * t97 * t8299;
    (t8370, t8373, t8374, t8376, t8381, t8382, t8386)
}
