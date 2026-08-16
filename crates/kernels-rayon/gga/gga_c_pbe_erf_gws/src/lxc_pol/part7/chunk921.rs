//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 921/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk921(t1827: f64, t4976: f64, t587: f64, t610: f64, t16699: f64, t1821: f64, t17246: f64, t17251: f64, t17255: f64, t17257: f64, t17259: f64, t17264: f64, t17267: f64, t17271: f64, t17275: f64) -> (f64, f64, f64) {
    let t17279 = 16.0_f64 / 45.0_f64 * t587 * t1827 * t4976 * t610;
    let t17282 = 32.0_f64 / 45.0_f64 * t587 * t1821 * t16699;
    let t17283 = t17246 + t17251 + t17255 - t17257 + t17259 - t17264 + t17267 - t17271 + t17275 - t17279 - t17282;
    (t17279, t17282, t17283)
}
