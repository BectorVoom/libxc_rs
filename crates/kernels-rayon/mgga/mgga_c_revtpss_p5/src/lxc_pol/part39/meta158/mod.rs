//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta158 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk716;
use chunk1::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk717;
use chunk2::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk718;
use chunk3::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk719;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta158(t3495: f64, t439: f64, t1187: f64, t1188: f64, t3356: f64, t3413: f64, t3358: f64, t3365: f64, t3370: f64, t3374: f64, t3392: f64, t3400: f64, t3408: f64, t3410: f64, t3415: f64, t3419: f64, t3422: f64, t3425: f64, t1178: f64, t447: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3496, t3497) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk716(t3495, t439, t1187);
        let (t3498, t3503, t3510, t3515) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk717(t1188, t3497, t3356, t3413, t3358, t3365, t3370, t3374, t3392, t3400, t3408, t3410, t3415, t3419, t3422, t3425);
        let (t3516, t3519, t3520) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk718(t1188, t3515, t1178);
        let (t3521, t3522, t3523) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk719(t3520, t439, t447);
    (t3496, t3497, t3498, t3503, t3510, t3515, t3516, t3519, t3520, t3521, t3522, t3523)
}
