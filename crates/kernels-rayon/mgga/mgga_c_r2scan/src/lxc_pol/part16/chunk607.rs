//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 607/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk607(t3347: f64, t797: f64, t1048: f64, t499: f64, t2333: f64, t795: f64, t3263: f64, t3275: f64, t321: f64, t502: f64, t263: f64, t818: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3348 = t3347 * t797;
    let t3350 = t1048 * t499 * t3348;
    let t3351 = t3350 / 4.0_f64;
    let t3352 = t2333 * t795;
    let t3354 = t3275 * t3263 * t3352;
    let t3355 = t3354 / 4.0_f64;
    let t3356 = t502 * t321;
    let t3357 = t3356 / 3.0_f64;
    let t3358 = t263 * t818;
    (t3348, t3351, t3352, t3355, t3357, t3358)
}
