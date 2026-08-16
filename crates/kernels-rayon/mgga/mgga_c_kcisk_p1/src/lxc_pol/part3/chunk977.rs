//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 977/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk977(t14393: f64, t1504: f64, t14345: f64, t14348: f64, t14351: f64, t14354: f64, t14357: f64, t14359: f64, t14361: f64, t14363: f64, t14368: f64, t14371: f64, t14377: f64, t14381: f64, t14388: f64, t14391: f64) -> (f64, f64) {
    let t14394 = t1504 * t14393;
    let t14396 = t14345 / 32.0_f64 - t14348 / 192.0_f64 - 19.0_f64 / 36.0_f64 * t14351 + t14354 / 4.0_f64 - 3.0_f64 / 128.0_f64 * t14357 + t14359 / 8.0_f64 - t14361 / 64.0_f64 + t14363 - 3.0_f64 / 8.0_f64 * t14368 + 11.0_f64 / 9.0_f64 * t14371 + t14377 / 864.0_f64 - 77.0_f64 / 27.0_f64 * t14381 + 209.0_f64 / 216.0_f64 * t14388 - t14391 / 8.0_f64 + 19.0_f64 / 48.0_f64 * t14394;
    (t14394, t14396)
}
