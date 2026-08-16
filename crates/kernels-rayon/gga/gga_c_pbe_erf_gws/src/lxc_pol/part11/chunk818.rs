//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 818/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk818(t13086: f64, t382: f64, t804: f64, t3780: f64, t829: f64, t830: f64, t831: f64, t1076: f64, t1109: f64, t1118: f64, t353: f64, t4386: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13087 = t382 * t13086;
    let t13088 = t804 * t13087;
    let t13096 = t829 * t830 * t831 * t3780;
    let t13105 = t829 * t830 * t831 * t1076;
    let t13110 = t1118 * t1109;
    let t13111 = t353 * t13110;
    let t13112 = t4386 * t13111;
    (t13087, t13088, t13096, t13105, t13110, t13112)
}
