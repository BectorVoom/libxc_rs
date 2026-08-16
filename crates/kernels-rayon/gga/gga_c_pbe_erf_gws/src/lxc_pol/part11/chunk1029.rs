//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1029/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1029(t1: f64, t3637: f64, t392: f64, t8122: f64, t12912: f64, t156: f64, t496: f64, t12958: f64, t395: f64, t485: f64, t12949: f64, t12952: f64, t501: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t42658 = t3637 * t1 * t392;
    let t42659 = t8122 * t42658;
    let t42661 = t156 * t12912;
    let t42662 = t496 * t42661;
    let t42665 = t485 * t12958 * t395;
    let t42672 = t485 * t12949 * t395;
    let t42675 = t501 * t12952 * t395;
    (t42658, t42659, t42661, t42662, t42665, t42672, t42675)
}
