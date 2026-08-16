//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 630/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk630(t1812: f64, t4913: f64, t1627: f64, t1817: f64, t1403: f64, t1764: f64, t562: f64, t1821: f64, t1820: f64, t1765: f64, t610: f64, t1827: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4915 = 16.0_f64 / 15.0_f64 * t4913 * t1812;
    let t4917 = 8.0_f64 / 15.0_f64 * t1627 * t1817;
    let t4919 = t562 * t1764 * t1403;
    let t4920 = t1821 * t4919;
    let t4922 = 16.0_f64 / 15.0_f64 * t1820 * t4920;
    let t4923 = t1765 * t610;
    let t4924 = t1827 * t4923;
    (t4915, t4917, t4919, t4920, t4922, t4923, t4924)
}
