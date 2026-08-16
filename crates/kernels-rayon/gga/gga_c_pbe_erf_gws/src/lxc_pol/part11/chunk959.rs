//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 959/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk959(t366: f64, t991: f64, t169: f64, t242: f64, t1076: f64, t413: f64, t1383: f64, t2994: f64, t1378: f64, t6056: f64, t922: f64, t281: f64, t285: f64, t4576: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t26036 = t366 * t991;
    let t26038 = t169 * t26036 * t242;
    let t26051 = t413 * t1076;
    let t26061 = t169 * t2994 * t1383;
    let t26085 = t922 * t991 * t1378 * t6056;
    let t26101 = t281 * t991 * t4576 * t285;
    (t26036, t26038, t26051, t26061, t26085, t26101)
}
