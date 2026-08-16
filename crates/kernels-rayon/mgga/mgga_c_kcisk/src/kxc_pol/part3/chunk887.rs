//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 887/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk887(t13288: f64, t1341: f64, t1415: f64, t1411: f64, t1390: f64, t382: f64, t1286: f64, t3278: f64, t3484: f64, t3482: f64, t1440: f64, t5625: f64) -> (f64, f64, f64, f64, f64) {
    let t13289 = t1341 * t13288;
    let t13290 = t1415 * t13289;
    let t13291 = t1411 * t13290;
    let t13293 = t382 * t1390;
    let t13294 = t3278 * t1286;
    let t13295 = t13293 * t13294;
    let t13296 = t3484 * t13295;
    let t13297 = t3482 * t13296;
    let t13299 = t3278 * t1440;
    let t13300 = t5625 * t13299;
    (t13291, t13294, t13297, t13299, t13300)
}
