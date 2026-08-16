//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1243/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1243(t10935: f64, t3162: f64, t3446: f64, t39245: f64, t39247: f64, t39252: f64, t39256: f64, t42395: f64, t42398: f64, t42402: f64, t42405: f64, t42408: f64, t42411: f64, t42415: f64, t42417: f64, t42422: f64, t42427: f64) -> f64 {
    let t43820 = t3446 * t10935 * t3162;
    let t43823 = -t42395 + t42398 - t42402 - t42405 - t42408 - t42411 + 0.96056421943322389208e-3_f64 * t43820 + t39245 - 0.72042316457491791906e-3_f64 * t39247 + t39252 + t39256 - t42415 + t42417 + t42422 - t42427;
    t43823
}
