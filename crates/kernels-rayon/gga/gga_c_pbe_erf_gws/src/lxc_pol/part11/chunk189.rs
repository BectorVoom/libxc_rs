//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 189/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk189(t103: f64, t485: f64, t395: f64, t118: f64, t119: f64, t120: f64, t155: f64, t117: f64, t4: f64) -> (f64, f64, f64, f64) {
    let t486 = t485 * t103;
    let t488 = 0.48717083333333333333e0_f64 * t486 * t395;
    let t495 = t118 * t119 * t155 * t120 / 12.0_f64;
    let t496 = t117 * t4;
    (t486, t488, t495, t496)
}
