//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1080/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1080(t338: f64, t4053: f64, t892: f64, t1176: f64, t2298: f64, t367: f64, t1178: f64, t2402: f64, t371: f64, t4052: f64, t810: f64, t2376: f64, t2409: f64) -> (f64, f64, f64, f64, f64) {
    let t13826 = t338 * t892 * t4053;
    let t13830 = t1176 * t367 * t2298;
    let t13832 = t371 * t1178 * t2402;
    let t13833 = t13830 * t13832;
    let t13835 = t4052 * t810;
    let t13837 = t2409 * t2376 * t13835;
    (t13826, t13832, t13833, t13835, t13837)
}
