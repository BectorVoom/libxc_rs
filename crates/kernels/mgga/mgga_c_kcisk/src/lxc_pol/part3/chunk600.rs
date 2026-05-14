//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 600/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk600<F: Float>(t10329: F, t1156: F, t4569: F, t294: F, t1008: F, t195: F, t1053: F, t3187: F, t1006: F, t3185: F, t494: F, t560: F, t1157: F, t3465: F, t3274: F, t3186: F, sigma0: F) -> (F, F, F, F, F, F, F) {
    let t10330 = 3.0 * t10329;
    let t10331 = t1156 * t4569;
    let t10332 = t294 * t10331;
    let t10333 = 3.0 / 16.0 * t10332;
    let t10334 = t1008 * t1008;
    let t10335 = 1.0 / t10334;
    let t10336 = t195 * t10335;
    let t10337 = t3187 * t1053;
    let t10338 = t10336 * t10337;
    let t10339 = 6.0 * t10338;
    let t10340 = t1006 * t3185;
    let t10341 = t10340 * t3187;
    let t10342 = 6.0 * t10341;
    let t10343 = 1.0 / t494;
    let t10344 = sigma0 * t10343;
    let t10345 = t10344 * t560;
    let t10346 = 3.0 / 8.0 * t10345;
    let t10347 = t3465 * t1157;
    let t10348 = 3.0 / 8.0 * t10347;
    let t10349 = t1053 * t3274;
    let t10350 = t3186 * t10349;
    (t10330, t10333, t10339, t10342, t10346, t10348, t10350)
}
