//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 462/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk462(t582: f64, t996: f64, t561: f64, t1006: f64, t583: f64, t1076: f64, t153: f64, t542: f64, t75: f64, t959: f64) -> (f64, f64, f64, f64, f64) {
    let t2796 = t582 * t996;
    let t2797 = t561 * t2796;
    let t2807 = t1006 * t583;
    let t2837 = t153 * t542 * t1076;
    let t2840 = t959 * t75;
    (t2796, t2797, t2807, t2837, t2840)
}
