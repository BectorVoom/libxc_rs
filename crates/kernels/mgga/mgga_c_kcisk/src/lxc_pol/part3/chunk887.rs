//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 887/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk887<F: Float>(t13288: F, t1341: F, t1415: F, t1411: F, t1390: F, t382: F, t1286: F, t3278: F, t3484: F, t3482: F, t1440: F, t5625: F) -> (F, F, F, F, F) {
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
