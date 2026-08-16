//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 597/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk597(t1393: f64, t514: f64, t185: f64, t1301: f64, t493: f64, t1288: f64, t548: f64, t1327: f64, t945: f64, t1326: f64, t1325: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3384 = t514 * t1393;
    let t3385 = t185 * t3384;
    let t3386 = 8.0_f64 / 15.0_f64 * t3385;
    let t3387 = t514 * t1301;
    let t3388 = t493 * t3387;
    let t3389 = 8.0_f64 / 15.0_f64 * t3388;
    let t3390 = t514 * t1288;
    let t3391 = t548 * t3390;
    let t3392 = 8.0_f64 / 15.0_f64 * t3391;
    let t3393 = t1327 * t945;
    let t3394 = t1326 * t3393;
    let t3396 = 8.0_f64 / 15.0_f64 * t1325 * t3394;
    (t3384, t3385, t3386, t3387, t3388, t3389, t3390, t3391, t3392, t3393, t3394, t3396)
}
