//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 316/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk316<F: Float>(t1302: F, t1303: F, t106: F, t78: F, t14: F, t60: F, t159: F, t88: F, t108: F, t348: F, t1147: F, t391: F, t1174: F, t70: F, t405: F, t105: F, t107: F, t1249: F, t438: F, t446: F, t447: F, t451: F, t73: F) -> (F, F, F, F, F) {
    let t1304 = t1302 * t1303;
    let t1308 = t78 * t106;
    let t1312 = t60 * t14;
    let t1319 = t159 * t88;
    let t1320 = t348 * t108;
    let t1326 = t391 * t1147;
    let t1330 = t70 * t1174;
    let t1334 = t405 * t405;
    let t1338 = -0.43802864444444444443e-3 * t105 * t1308 * t108 - 0.2e-22 * t446 * t1312 * t108 - 0.26281718666666666666e-2 * t105 * t438 * t451 + 0.19711288999999999999e-2 * t1319 * t1320 + 0.19711288999999999999e-2 * t446 * t447 * t451 + 0.39422577999999999998e-2 * t105 * t107 * t1326 - 0.19711288999999999999e-2 * t105 * t107 * t1330 - 4.0 * t1334 - 4.0 * t73 * t1249;
    (t1304, t1308, t1312, t1319, t1338)
}
