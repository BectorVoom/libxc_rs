//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 964/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk964(t5975: f64, t992: f64, t1964: f64, t2519: f64, t11274: f64, t475: f64, t1076: f64, t169: f64, t301: f64, t922: f64, t1368: f64, t285: f64, t3013: f64) -> (f64, f64, f64, f64, f64) {
    let t26437 = t992 * t5975;
    let t26439 = t2519 * t1964;
    let t26470 = t475 * t11274;
    let t26477 = t169 * t922 * t1076 * t301;
    let t26480 = t3013 * t1368 * t285;
    (t26437, t26439, t26470, t26477, t26480)
}
