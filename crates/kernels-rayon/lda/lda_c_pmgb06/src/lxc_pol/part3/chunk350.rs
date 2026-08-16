//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 350/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk350(t1234: f64, t1282: f64, t1227: f64, t1241: f64, t1247: f64, t1249: f64, t1252: f64, t1255: f64, t1259: f64, t1261: f64, t1264: f64, t1268: f64, t1274: f64, t1277: f64, t1280: f64, t360: f64, t370: f64, t63: f64) -> f64 {
    let t1283 = t1282 * t1234;
    let t1289 = -t1241 + t1247 + t1249 + t1252 - t1255 + t1259 + t1261 / 3.0_f64 + 3.0_f64 / 2.0_f64 * t360 * t1264 - t360 * t1268 / 2.0_f64 + t1274 + 1.46904_f64 * t1277 + t1280 + 5.87616_f64 * t63 * t1283 - 1.46904_f64 * t63 * t370 * t1227;
    t1289
}
