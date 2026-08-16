//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1137/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1137(t10610: f64, t3276: f64, t42432: f64, t3579: f64, t40649: f64, t42383: f64, t42387: f64, t42391: f64, t42395: f64, t42398: f64, t42402: f64, t42405: f64, t42408: f64, t42411: f64, t42415: f64, t42417: f64, t42422: f64, t42427: f64, t42431: f64) -> (f64, f64, f64) {
    let t42435 = 15.0_f64 / 8.0_f64 * t10610 * t3276 * t42432;
    let t42437 = t3579 * t40649 / 2.0_f64;
    let t42438 = -t42383 - t42387 + t42391 + t42395 - t42398 + t42402 + t42405 + t42408 + t42411 + t42415 - t42417 - t42422 + t42427 + t42431 - t42435 - t42437;
    (t42435, t42437, t42438)
}
