//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 835/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk835<F: Float>(t1325: F, t6230: F, t2396: F, t3802: F, t519: F, t2388: F, t3863: F, t571: F, t2384: F, t3854: F, t1318: F, t811: F, t833: F, t593: F, t5269: F, t2035: F, t4763: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t6232 = 16.0 / 45.0 * t1325 * t6230;
    let t6233 = t3802 * t2396;
    let t6234 = t519 * t6233;
    let t6235 = 16.0 / 135.0 * t6234;
    let t6236 = t3863 * t2388;
    let t6237 = t571 * t6236;
    let t6238 = 16.0 / 135.0 * t6237;
    let t6239 = t3854 * t2384;
    let t6240 = t1318 * t6239;
    let t6241 = 32.0 / 135.0 * t6240;
    let t6242 = t811 * t833;
    let t6243 = t6242 * t593;
    let t6244 = t5269 * t6243;
    let t6246 = 16.0 / 15.0 * t1318 * t6244;
    let t6248 = 16.0 / 45.0 * t4763 * t2035;
    (t6232, t6233, t6235, t6236, t6238, t6239, t6241, t6242, t6243, t6244, t6246, t6248)
}
