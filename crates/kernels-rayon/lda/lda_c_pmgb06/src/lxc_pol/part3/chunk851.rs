//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 851/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk851(t110: f64, t3589: f64, t360: f64, t1263: f64, t410: f64, t1271: f64, t1282: f64, t8299: f64, t97: f64, t3566: f64, t8305: f64, t1276: f64, t8373: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8378 = t110 * t3589;
    let t8379 = t360 * t8378;
    let t8381 = t410 * t1263;
    let t8382 = t360 * t8381;
    let t8386 = t1271 * t1282 * t97 * t8299;
    let t8388 = t3566 * t8305;
    let t8390 = t1276 * t8373;
    (t8378, t8379, t8381, t8382, t8386, t8388, t8390)
}
