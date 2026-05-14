//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 563/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk563<F: Float>(t3391: F, t1327: F, t945: F, t1326: F, t1325: F, t1245: F, t494: F, t940: F, t1991: F, t1459: F, t529: F, t1246: F, t542: F, t519: F, t1252: F, t1251: F, t348: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t3392 = 8.0 / 15.0 * t3391;
    let t3393 = t1327 * t945;
    let t3394 = t1326 * t3393;
    let t3396 = 8.0 / 15.0 * t1325 * t3394;
    let t3398 = t494 * t1245 * t940;
    let t3399 = t1991 * t3398;
    let t3401 = 8.0 / 9.0 * t1325 * t3399;
    let t3402 = t1459 * t529;
    let t3403 = t1246 * t542;
    let t3404 = t3402 * t3403;
    let t3406 = 4.0 / 9.0 * t519 * t3404;
    let t3407 = t1252 * t494;
    let t3408 = t1326 * t3407;
    let t3410 = 16.0 / 15.0 * t1325 * t3408;
    let t3411 = t1251 * t348;
    (t3392, t3393, t3394, t3396, t3398, t3399, t3401, t3402, t3403, t3404, t3406, t3407, t3408, t3410, t3411)
}
