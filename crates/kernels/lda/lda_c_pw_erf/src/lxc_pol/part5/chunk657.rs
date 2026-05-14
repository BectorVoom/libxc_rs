//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 657/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk657<F: Float>(t348: F, t6280: F, t1313: F, t519: F, t2526: F, t558: F, t352: F, t1308: F, t571: F, t2002: F, t4763: F, t2392: F, t3859: F, t1325: F, t2123: F, t822: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t6281 = t6280 * t348;
    let t6282 = t1313 * t6281;
    let t6284 = 4.0 / 45.0 * t519 * t6282;
    let t6285 = t2526 * t558;
    let t6286 = t6285 * t352;
    let t6287 = t1308 * t6286;
    let t6289 = 4.0 / 45.0 * t571 * t6287;
    let t6291 = 16.0 / 45.0 * t4763 * t2002;
    let t6292 = t3859 * t2392;
    let t6293 = t1325 * t6292;
    let t6294 = 32.0 / 135.0 * t6293;
    let t6295 = t822 * t2123;
    (t6281, t6282, t6284, t6285, t6286, t6287, t6289, t6291, t6292, t6293, t6294, t6295)
}
