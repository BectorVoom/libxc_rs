//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 474/988 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk474<F: Float>(t60: F, t3998: F, t525: F, t50: F, t921: F, t284: F, t814: F, t1403: F, t1406: F, t154: F, t62: F, t922: F, t925: F, t5338: F, t277: F, t1392: F, t500: F, zeta_threshold: F) -> (F, F, F, F) {
    let t61 = t60 <= zeta_threshold;
    let t5339 = t3998 * t525;
    let t5342 = t921 * t50;
    let t5343 = t814 * t284;
    let t5353 = piecewise3(t61, 0.0, -8.0 / 27.0 * t5339 * t922 - 16.0 / 9.0 * t5342 * t5343 + 4.0 / 9.0 * t1403 * t925 - 8.0 / 3.0 * t62 * t814 + 8.0 * t1406 * t154);
    let t5354 = t5338 + t5353;
    let t5355 = t277 * t5354;
    let t5372 = t500 * t1392;
    (t5343, t5354, t5355, t5372)
}
