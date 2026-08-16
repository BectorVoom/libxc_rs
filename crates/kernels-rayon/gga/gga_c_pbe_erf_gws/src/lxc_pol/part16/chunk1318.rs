//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1318/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1318(t27047: f64, t3067: f64, t4216: f64, t814: f64, t1205: f64, t26654: f64, t829: f64, t830: f64, t4083: f64, t8746: f64, t2416: f64, t4227: f64) -> (f64, f64, f64, f64) {
    let t55137 = t27047 * t3067 * t4216 * t814;
    let t55140 = t26654 * t1205;
    let t55142 = t829 * t830 * t55140;
    let t55145 = t8746 * t4083;
    let t55151 = t2416 * t4227;
    (t55137, t55142, t55145, t55151)
}
