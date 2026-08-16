//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 920/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk920(t7993: f64, t8006: f64, t8025: f64, t8036: f64, t242: f64, t3013: f64, t153: f64, t2848: f64, t542: f64, t145: f64, t2522: f64, t2519: f64, t700: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8038 = t7993 + t8006 + t8025 + t8036;
    let t8042 = t3013 * t242;
    let t8047 = 0.11389037339096724978e1_f64 * t153 * t542 * t2848;
    let t8048 = t145 * t2522;
    let t8050 = 0.16752564107100880375e0_f64 * t8048 * t242;
    let t8051 = t2519 * t700;
    (t8038, t8042, t8047, t8048, t8050, t8051)
}
