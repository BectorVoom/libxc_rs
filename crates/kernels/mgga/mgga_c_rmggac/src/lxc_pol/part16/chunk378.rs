//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 378/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk378<F: Float>(t2350: F, t797: F, t2347: F, t262: F, t2100: F, t2103: F, t851: F, t854: F, t2115: F, t2118: F, t511: F, t623: F, t2295: F, t793: F, t2298: F, t2301: F, t305: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t2351 = t797 * t2350;
    let t2353 = t262 * t2347;
    let t2354 = t2100 * t2353;
    let t2356 = t262 * t2350;
    let t2357 = t2103 * t2356;
    let t2359 = t851 * t2347;
    let t2361 = t854 * t2350;
    let t2363 = t2115 * t2353;
    let t2365 = t2118 * t2356;
    let t2373 = t623 * t511;
    let t2382 = t793 * t2295;
    let t2384 = t797 * t2298;
    let t2386 = t305 * t2301;
    (t2351, t2353, t2354, t2356, t2357, t2359, t2361, t2363, t2365, t2373, t2382, t2384, t2386)
}
