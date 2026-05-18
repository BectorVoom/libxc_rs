//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 387/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk387<F: Float>(t2100: F, t2353: F, t2350: F, t262: F, t2103: F, t2347: F, t851: F, t854: F, t2115: F, t2118: F, t511: F, t623: F) -> (F, F, F, F, F, F, F, F) {
    let t2354 = t2100 * t2353;
    let t2356 = t262 * t2350;
    let t2357 = t2103 * t2356;
    let t2359 = t851 * t2347;
    let t2361 = t854 * t2350;
    let t2363 = t2115 * t2353;
    let t2365 = t2118 * t2356;
    let t2373 = t623 * t511;
    (t2354, t2356, t2357, t2359, t2361, t2363, t2365, t2373)
}
