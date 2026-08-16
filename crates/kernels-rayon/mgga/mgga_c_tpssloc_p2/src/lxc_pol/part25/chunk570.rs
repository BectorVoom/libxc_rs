//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 570/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk570(t1230: f64, t248: f64, t3252: f64, t3248: f64, t1009: f64, t1190: f64, t1011: f64, t1212: f64, t374: f64, t486: f64, t677: f64, t485: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3527 = t248 * t1230 * t3252;
    let t3531 = t248 * t1230 * t3248;
    let t3534 = t1190 * t1009;
    let t3535 = t3534 * t1011;
    let t3536 = t3535 * t1212;
    let t3540 = t374 * t677 * t486;
    let t3542 = t485 * t3540 / 13824.0_f64;
    (t3527, t3531, t3534, t3536, t3540, t3542)
}
