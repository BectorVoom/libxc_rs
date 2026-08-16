//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 960/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk960(t1083: f64, t5631: f64, t1473: f64, t2936: f64, t1503: f64, t8496: f64, t1371: f64, t1480: f64, t8308: f64, t413: f64, t991: f64, t159: f64, t285: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t26118 = t5631 * t1083;
    let t26129 = t1473 * t2936;
    let t26131 = t1503 * t8496;
    let t26135 = t8308 * t1371 * t1480;
    let t26143 = t413 * t991;
    let t26145 = t26143 * t159 * t285;
    (t26118, t26129, t26131, t26135, t26143, t26145)
}
