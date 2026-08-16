//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 640/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk640(t7778: f64, t7782: f64, t7820: f64, t8195: f64, t7771: f64, t8189: f64, t1851: f64, t480: f64, t1827: f64, t1882: f64, t494: f64, t8232: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8444 = t7778 / 9.0_f64;
    let t8446 = 2.0_f64 / 27.0_f64 * t7782;
    let t8449 = 2.0_f64 / 9.0_f64 * t7820;
    let t8452 = t8195 / 3.0_f64;
    let t8454 = 2.0_f64 / 3.0_f64 * t7771;
    let t8455 = 28.0_f64 / 81.0_f64 * t8189;
    let t8466 = t480 * t1851;
    let t8471 = t1882 * t1827;
    let t8475 = t8232 * t494;
    (t8444, t8446, t8449, t8452, t8454, t8455, t8466, t8471, t8475)
}
