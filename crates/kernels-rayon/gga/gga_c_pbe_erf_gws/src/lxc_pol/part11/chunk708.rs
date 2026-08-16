//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 708/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk708(t3361: f64, t409: f64, t414: f64, t153: f64, t3373: f64, t542: f64, t3488: f64, t583: f64, t1630: f64, t3499: f64, t639: f64, t181: f64, t995: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10257 = t409 * t3361;
    let t10259 = t414 * t3361;
    let t10283 = t153 * t542 * t3373;
    let t10293 = t3488 * t583;
    let t10300 = t1630 * t3499;
    let t10301 = t639 * t10300;
    let t10325 = t995 * t181;
    (t10257, t10259, t10283, t10293, t10300, t10301, t10325)
}
