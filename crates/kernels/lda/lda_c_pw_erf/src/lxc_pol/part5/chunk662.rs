//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 662/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk662<F: Float>(t1991: F, t6352: F, t519: F, t2419: F, t593: F, t1308: F, t571: F, t1333: F, t2337: F, t352: F, t1319: F, t1351: F, t2017: F, t2411: F, t549: F, t1318: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t6353 = t1991 * t6352;
    let t6355 = 8.0 / 9.0 * t519 * t6353;
    let t6356 = t2419 * t593;
    let t6357 = t1308 * t6356;
    let t6359 = 4.0 / 45.0 * t571 * t6357;
    let t6360 = t1333 * t2337;
    let t6361 = t6360 * t352;
    let t6362 = t1319 * t6361;
    let t6364 = 8.0 / 45.0 * t571 * t6362;
    let t6365 = t1351 * t2337;
    let t6366 = t6365 * t352;
    let t6367 = t2017 * t6366;
    let t6369 = 4.0 / 27.0 * t571 * t6367;
    let t6370 = t2411 * t549;
    let t6371 = t2017 * t6370;
    let t6373 = 8.0 / 27.0 * t1318 * t6371;
    (t6353, t6355, t6356, t6357, t6359, t6360, t6361, t6362, t6364, t6365, t6366, t6367, t6369, t6370, t6371, t6373)
}
