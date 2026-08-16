//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 446/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk446(t2355: f64, t2493: f64, t2: f64, t2360: f64, t2349: f64, t737: f64, t1934: f64, t738: f64, t2371: f64, t192: f64, t2373: f64, t2459: f64, t743: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2494 = t2493 * t2355;
    let t2497 = t2 * t2360;
    let t2498 = t2497 * t2349;
    let t2499 = t737 * t2498;
    let t2502 = t738 * t1934;
    let t2503 = t737 * t2502;
    let t2506 = t2371 * t2;
    let t2508 = t192 * t2506 * t2373;
    let t2512 = t192 * t743 * t2459;
    (t2494, t2498, t2499, t2502, t2503, t2506, t2508, t2512)
}
