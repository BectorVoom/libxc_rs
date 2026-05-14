//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 648/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk648<F: Float>(t447: F, t4623: F, t925: F, t4589: F, t942: F, t452: F, t488: F, t110: F, t20035: F, t16052: F, t1902: F, t20208: F, t3187: F, t11846: F, t11883: F, t16255: F, t16296: F, t16298: F, t16300: F, t16302: F, t16490: F, t16539: F, t1901: F, t446: F, t8534: F) -> (F, F, F, F, F, F, F, F, F) {
    let t20276 = t447 * t4623 * t925;
    let t20279 = t942 * t4589;
    let t20281 = t452 * t488 * t20279;
    let t20284 = t447 * t110 * t20035;
    let t20287 = t16052 * t925;
    let t20288 = t1902 * t20287;
    let t20291 = t3187 * t20208;
    let t20292 = t1902 * t20291;
    let t20304 = -t446 * t20276 / 3.0 + t446 * t20281 - 2.0 / 3.0 * t446 * t20284 + t1901 * t20288 / 3.0 - 2.0 / 3.0 * t1901 * t20292 - 2.0 / 9.0 * t16255 - 4.0 / 9.0 * t11846 - 2.0 / 3.0 * t16296 + 2.0 / 27.0 * t16298 + t16300 / 9.0 + 2.0 / 9.0 * t16302 - 4.0 / 27.0 * t11883 - t8534 - 2.0 / 3.0 * t16490 - 2.0 / 3.0 * t16539;
    (t20276, t20279, t20281, t20284, t20287, t20288, t20291, t20292, t20304)
}
