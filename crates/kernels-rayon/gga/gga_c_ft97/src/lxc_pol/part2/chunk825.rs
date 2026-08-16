//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 825/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk825(t3565: f64, t558: f64, t574: f64, t605: f64, t3541: f64, t376: f64, t89: f64, t1882: f64, t3452: f64, t3457: f64, t157: f64, t1985: f64) -> (f64, f64, f64, f64, f64) {
    let t12956 = t3565 * t558;
    let t12958 = t574 * t605 * t12956;
    let t12963 = 2.0_f64 / 9.0_f64 * t89 * t376 * t3541;
    let t12965 = 4.0_f64 / 9.0_f64 * t1882 * t3452;
    let t12967 = 2.0_f64 / 9.0_f64 * t1882 * t3457;
    let t12968 = t1985 * t157;
    (t12958, t12963, t12965, t12967, t12968)
}
