//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1308/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1308(t258: f64, t39552: f64, t2454: f64, t2455: f64, t39494: f64, t14545: f64, t251: f64, t786: f64, t2710: f64, t2793: f64, t211: f64, t9644: f64) -> (f64, f64, f64, f64, f64) {
    let t39554 = 0.88356352675825229576e-3_f64 * t39552 * t258;
    let t39557 = 0.20561456923286030469e-1_f64 * t2454 * t2455 * t39494;
    let t39597 = t14545 * t251;
    let t39598 = t786 * t39597;
    let t39633 = 0.20561456923286030469e-1_f64 * t2710 * t2793 * t39494;
    let t39643 = 1.0_f64 / t9644 / t211;
    (t39554, t39557, t39598, t39633, t39643)
}
