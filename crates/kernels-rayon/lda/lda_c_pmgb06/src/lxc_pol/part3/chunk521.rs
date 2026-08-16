//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 521/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk521(t1241: f64, t1249: f64, t1259: f64, t1261: f64, t1274: f64, t1277: f64, t1280: f64, t2185: f64, t2188: f64, t2191: f64, t2212: f64, t2215: f64, t2217: f64, t2222: f64, t2227: f64, t2229: f64, t2233: f64, t342: f64, t35: f64, t360: f64, t63: f64) -> f64 {
    let t2236 = -t1241 + t2185 + t1249 + t2188 + t2191 - t2212 + t1259 + t1261 / 6.0_f64 + t2215 / 6.0_f64 + 3.0_f64 / 2.0_f64 * t360 * t35 * t2217 - t360 * t2222 / 2.0_f64 + t1274 + 0.73452_f64 * t1277 + t1280 + 0.73452_f64 * t2227 + 5.87616_f64 * t63 * t2229 * t342 - 1.46904_f64 * t63 * t2233;
    t2236
}
