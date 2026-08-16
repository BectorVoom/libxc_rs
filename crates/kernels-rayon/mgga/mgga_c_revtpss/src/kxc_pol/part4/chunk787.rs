//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 787/1428 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk787(t36: f64, t4186: f64, t70: f64, t1470: f64, t627: f64, t1486: f64, t607: f64, t1469: f64, t2275: f64, t606: f64, t48: f64, t2282: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4187 = t36 * t4186;
    let t4188 = t4187 * t70;
    let t4191 = t1470 * t627;
    let t4196 = t607 * t1486;
    let t4201 = t2275 * t1469;
    let t4202 = t4201 * t606;
    let t4205 = t48 * t4186;
    let t4210 = t2282 * t1469;
    (t4187, t4188, t4191, t4196, t4201, t4202, t4205, t4210)
}
