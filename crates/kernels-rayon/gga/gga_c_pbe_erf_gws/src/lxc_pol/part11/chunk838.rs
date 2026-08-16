//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 838/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk838(t13252: f64, t3221: f64, t11539: f64, t1109: f64, t3752: f64, t3258: f64, t2255: f64, t1076: f64, t1133: f64, t343: f64, t1123: f64, t274: f64, t3854: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t13253 = t3221 * t13252;
    let t13254 = t11539 * t13253;
    let t13257 = t3752 * t1109;
    let t13258 = t3258 * t13257;
    let t13259 = t2255 * t13258;
    let t13262 = t1076 * t1133;
    let t13263 = t13262 * t343;
    let t13264 = t1123 * t13263;
    let t13265 = t2255 * t13264;
    let t13269 = t274 * t3854 * t343;
    (t13253, t13254, t13257, t13259, t13263, t13265, t13269)
}
