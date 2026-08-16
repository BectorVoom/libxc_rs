//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 316/1308 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk316(t1174: f64, t70: f64, t405: f64, t105: f64, t107: f64, t108: f64, t1249: f64, t1308: f64, t1312: f64, t1319: f64, t1320: f64, t1326: f64, t438: f64, t446: f64, t447: f64, t451: f64, t73: f64) -> f64 {
    let t1330 = t70 * t1174;
    let t1334 = t405 * t405;
    let t1338 = -0.43802864444444444443e-3_f64 * t105 * t1308 * t108 - 0.2e-22_f64 * t446 * t1312 * t108 - 0.26281718666666666666e-2_f64 * t105 * t438 * t451 + 0.19711288999999999999e-2_f64 * t1319 * t1320 + 0.19711288999999999999e-2_f64 * t446 * t447 * t451 + 0.39422577999999999998e-2_f64 * t105 * t107 * t1326 - 0.19711288999999999999e-2_f64 * t105 * t107 * t1330 - 4.0_f64 * t1334 - 4.0_f64 * t73 * t1249;
    t1338
}
