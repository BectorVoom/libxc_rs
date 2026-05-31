//! GGA_C_GAPC lxc pol — lxc_pol part 27 (v4rho2sigma2_6) CSE chunk 318/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part27_v4rho2sigma2_6_chunk318<F: Float>(t1174: F, t70: F, t405: F, t105: F, t107: F, t108: F, t1249: F, t1308: F, t1312: F, t1319: F, t1320: F, t1326: F, t438: F, t446: F, t447: F, t451: F, t73: F) -> F {
    let t1330 = t70 * t1174;
    let t1334 = t405 * t405;
    let t1338 = -F::cast_from(0.43802864444444444443e-3_f64) * t105 * t1308 * t108 - F::cast_from(0.2e-22_f64) * t446 * t1312 * t108 - F::cast_from(0.26281718666666666666e-2_f64) * t105 * t438 * t451 + F::cast_from(0.19711288999999999999e-2_f64) * t1319 * t1320 + F::cast_from(0.19711288999999999999e-2_f64) * t446 * t447 * t451 + F::cast_from(0.39422577999999999998e-2_f64) * t105 * t107 * t1326 - F::cast_from(0.19711288999999999999e-2_f64) * t105 * t107 * t1330 - F::cast_from(4.0_f64) * t1334 - F::cast_from(4.0_f64) * t73 * t1249;
    t1338
}
