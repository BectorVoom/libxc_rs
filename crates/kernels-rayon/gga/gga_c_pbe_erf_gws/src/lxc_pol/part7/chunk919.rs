//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 919/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk919(t4888: f64, t5312: f64, t1820: f64, t1821: f64, t418: f64, t5333: f64, t572: f64, t1651: f64, t1802: f64, t5550: f64, t587: f64, t1829: f64, t5304: f64) -> (f64, f64, f64, f64) {
    let t17246 = 32.0_f64 / 15.0_f64 * t5312 * t4888;
    let t17251 = 32.0_f64 / 45.0_f64 * t1820 * t1821 * t5333 * t572 * t418;
    let t17252 = t1651 * t1802;
    let t17254 = t587 * t17252 * t5550;
    let t17255 = 64.0_f64 / 45.0_f64 * t17254;
    let t17257 = 32.0_f64 / 15.0_f64 * t5304 * t1829;
    (t17246, t17251, t17255, t17257)
}
