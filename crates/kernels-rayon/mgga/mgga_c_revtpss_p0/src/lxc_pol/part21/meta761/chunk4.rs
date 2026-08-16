//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2700/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2700(t1353: f64, t4135: f64, t14304: f64, t1450: f64, t1448: f64, t47109: f64, t47116: f64, t47118: f64, t47122: f64, t47124: f64, t48315: f64, t48316: f64, t48317: f64, t48318: f64, t48319: f64, t48320: f64) -> (f64, f64, f64, f64) {
    let t49640 = t4135 * t1353;
    let t49647 = t14304 * t1450;
    let t49654 = t1448 * t4135;
    let t49659 = -t48315 - t47109 - t48316 + t48317 + t47116 - t47118 - t48318 + t47122 + t47124 + t48319 + t48320;
    (t49640, t49647, t49654, t49659)
}
