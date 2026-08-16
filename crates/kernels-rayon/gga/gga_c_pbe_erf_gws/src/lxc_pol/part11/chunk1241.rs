//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1241/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1241(t38850: f64, t45235: f64, t860: f64, t3793: f64, t44902: f64, t45228: f64, t44972: f64, t45240: f64, t37380: f64, t11478: f64, t13408: f64, t2168: f64, t6523: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t49658 = t45235 * t38850 * t860 / 16.0_f64;
    let t49660 = t44902 * t3793 / 32.0_f64;
    let t49661 = 7.0_f64 / 12.0_f64 * t45228;
    let t49663 = t44972 * t3793 / 16.0_f64;
    let t49664 = 7.0_f64 / 36.0_f64 * t45240;
    let t49667 = 35.0_f64 / 18.0_f64 * t37380;
    let t49671 = 3.0_f64 / 8.0_f64 * t2168 * t6523 * t11478 * t13408;
    (t49658, t49660, t49661, t49663, t49664, t49667, t49671)
}
