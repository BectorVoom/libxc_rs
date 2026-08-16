//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta1023 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3573;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3574;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3575;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3576;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3577;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3578;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3579;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1023(t20294: f64, t689: f64, t12256: f64, t2251: f64, t5825: f64, t12305: f64, t128: f64, t20297: f64, t1120: f64, t13312: f64, t5051: f64, t20319: f64, t18281: f64, t3362: f64, t606: f64, t3360: f64, t43771: f64, t43781: f64, t43783: f64, t43814: f64, t43817: f64, t68253: f64, t68255: f64, t68257: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t68262 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3573(t20294, t689);
        let (t68265, t68267) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3574(t12256, t2251, t5825, t12305, t128);
        let (t68269, t68271) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3575(t20297, t2251, t1120, t128);
        let (t68273, t68275) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3576(t13312, t5051, t1120, t128);
        let t68277 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3577(t20319, t689);
        let (t68280, t68282) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3578(t18281, t3362, t606, t128, t3360);
        let t68284 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3579(t43771, t43781, t43783, t43814, t43817, t68253, t68255, t68257, t68262, t68267, t68271, t68275, t68277, t68282);
    (t68262, t68265, t68267, t68269, t68271, t68273, t68275, t68277, t68280, t68282, t68284)
}
