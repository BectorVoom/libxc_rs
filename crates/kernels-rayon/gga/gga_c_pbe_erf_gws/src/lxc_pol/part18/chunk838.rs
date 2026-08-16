//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 838/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk838(t4857: f64, t4860: f64, t242: f64, t3013: f64, t153: f64, t2848: f64, t542: f64, t145: f64, t2522: f64, t2519: f64, t700: f64, t2523: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8034 = 48.0_f64 * t4857;
    let t8035 = 80.0_f64 * t4860;
    let t8042 = t3013 * t242;
    let t8047 = 0.11389037339096724978e1_f64 * t153 * t542 * t2848;
    let t8048 = t145 * t2522;
    let t8050 = 0.16752564107100880375e0_f64 * t8048 * t242;
    let t8051 = t2519 * t700;
    let t8057 = 0.16752564107100880375e0_f64 * t2523 * t700;
    (t8034, t8035, t8042, t8047, t8048, t8050, t8051, t8057)
}
