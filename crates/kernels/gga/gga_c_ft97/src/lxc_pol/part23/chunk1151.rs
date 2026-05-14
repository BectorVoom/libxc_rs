//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1151/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1151<F: Float>(t1882: F, t28116: F, t6927: F, t8232: F, t28291: F, t8392: F, t28201: F, t28110: F, t28167: F, t1443: F, t9952: F, t6875: F, t6154: F, t737: F, t28198: F, t28154: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t111405 = 2.0 / 9.0 * t1882 * t28116;
    let t111420 = t8232 * t6927;
    let t111436 = 2.0 / 27.0 * t8392 * t28291;
    let t111443 = 2.0 / 9.0 * t1882 * t28201;
    let t111452 = 2.0 / 9.0 * t1882 * t28110;
    let t111466 = 2.0 / 9.0 * t1882 * t28167;
    let t111478 = t9952 * t1443;
    let t111512 = t8232 * t6875;
    let t111518 = t737 * t6154;
    let t111523 = 2.0 / 9.0 * t1882 * t28198;
    let t111528 = 2.0 / 27.0 * t8392 * t28154;
    (t111405, t111420, t111436, t111443, t111452, t111466, t111478, t111512, t111518, t111523, t111528)
}
