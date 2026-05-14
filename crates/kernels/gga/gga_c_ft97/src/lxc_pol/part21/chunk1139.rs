//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1139/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1139<F: Float>(t1286: F, t29744: F, t376: F, t16198: F, t91493: F, t16203: F, t22943: F, t102921: F, t3219: F, t103163: F, t979: F, t1852: F, t26113: F, t4589: F, t5743: F, t1332: F, t16480: F) -> (F, F, F, F, F, F, F, F) {
    let t116091 = t1286 * t376 * t29744;
    let t116093 = t91493 * t16198;
    let t116095 = t22943 * t16203;
    let t116097 = t102921 * t3219;
    let t116099 = t103163 * t979;
    let t116102 = t1852 * t26113 * t979;
    let t116105 = t1852 * t5743 * t4589;
    let t116108 = t1852 * t1332 * t16480;
    (t116091, t116093, t116095, t116097, t116099, t116102, t116105, t116108)
}
