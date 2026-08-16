//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 347/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk347(t110: f64, t361: f64, t360: f64, t1234: f64, t370: f64, t35: f64, t1227: f64, t64: f64, t347: f64, t61: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t1260 = t110 * t361;
    let t1261 = t360 * t1260;
    let t1263 = t370 * t1234;
    let t1264 = t35 * t1263;
    let t1267 = t64 * t1227;
    let t1268 = t35 * t1267;
    let t1271 = t61 * t347;
    (t1260, t1261, t1263, t1264, t1267, t1268, t1271)
}
