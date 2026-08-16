//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 136/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk136(t41: f64, t425: f64, t68: f64, t63: f64, t390: f64, t393: f64, t398: f64, t388: f64, t71: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t426 = t41 * t425;
    let t430 = t68 * t68;
    let t431 = 1.0_f64 / t430;
    let t432 = t63 * t431;
    let t434 = 0.516475e0_f64 * t390;
    let t435 = 0.2103875e0_f64 * t393;
    let t436 = 0.104195e0_f64 * t398;
    let t437 = -0.1176575e1_f64 * t388 - t434 - t435 - t436;
    let t438 = 1.0_f64 / t71;
    (t426, t430, t431, t432, t434, t435, t436, t437, t438)
}
