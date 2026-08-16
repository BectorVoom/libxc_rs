//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 644/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk644(t1053: f64, t3187: f64, t10336: f64, t1006: f64, t3185: f64, t494: f64, t560: f64, t1157: f64, t3465: f64, t3274: f64, t3186: f64, t1152: f64, t4570: f64, sigma0: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10337 = t3187 * t1053;
    let t10338 = t10336 * t10337;
    let t10339 = 6.0_f64 * t10338;
    let t10340 = t1006 * t3185;
    let t10341 = t10340 * t3187;
    let t10342 = 6.0_f64 * t10341;
    let t10343 = 1.0_f64 / t494;
    let t10344 = sigma0 * t10343;
    let t10345 = t10344 * t560;
    let t10346 = 3.0_f64 / 8.0_f64 * t10345;
    let t10347 = t3465 * t1157;
    let t10348 = 3.0_f64 / 8.0_f64 * t10347;
    let t10349 = t1053 * t3274;
    let t10350 = t3186 * t10349;
    let t10351 = 6.0_f64 * t10350;
    let t10352 = t1152 * t4570;
    (t10339, t10342, t10346, t10348, t10351, t10352)
}
