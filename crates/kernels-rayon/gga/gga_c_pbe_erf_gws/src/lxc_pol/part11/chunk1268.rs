//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1268/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1268(t1109: f64, t11514: f64, t2255: f64, t2277: f64, t29599: f64, t3235: f64, t3258: f64, t3373: f64, t46253: f64, t46280: f64, t50069: f64, t50187: f64, t50189: f64, t50193: f64, t50201: f64, t50206: f64, t50207: f64, t9425: f64) -> f64 {
    let t50208 = -3.0_f64 / 64.0_f64 * t9425 * t3235 * t11514 * t50069 - t50187 - 7.0_f64 / 48.0_f64 * t46253 + t50189 + t50193 - t2277 * t2255 * t3258 * t3373 * t1109 / 512.0_f64 + 7.0_f64 / 96.0_f64 * t46280 - t50201 + 595.0_f64 / 1296.0_f64 * t29599 - t50206 - t50207;
    t50208
}
