//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta158 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk717;
use chunk1::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk718;
use chunk2::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk719;
use chunk3::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk720;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta158<F: Float>(t3495: F, t439: F, t1187: F, t1188: F, t3356: F, t3413: F, t3358: F, t3365: F, t3370: F, t3374: F, t3392: F, t3400: F, t3408: F, t3410: F, t3415: F, t3419: F, t3422: F, t3425: F, t1178: F, t447: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t3496, t3497) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk717::<F>(t3495, t439, t1187);
        let (t3498, t3503, t3510, t3515) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk718::<F>(t1188, t3497, t3356, t3413, t3358, t3365, t3370, t3374, t3392, t3400, t3408, t3410, t3415, t3419, t3422, t3425);
        let (t3516, t3519, t3520) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk719::<F>(t1188, t3515, t1178);
        let (t3521, t3522, t3523) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk720::<F>(t3520, t439, t447);
    (t3496, t3497, t3498, t3503, t3510, t3515, t3516, t3519, t3520, t3521, t3522, t3523)
}
