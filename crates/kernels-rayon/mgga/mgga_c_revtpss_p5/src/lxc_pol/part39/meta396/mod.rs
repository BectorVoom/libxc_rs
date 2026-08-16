//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta396 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1434;
use chunk1::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1435;
use chunk2::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1436;
use chunk3::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1437;
use chunk4::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1438;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta396(t17633: f64, t3629: f64, t3626: f64, t2258: f64, t3628: f64, t5351: f64, t3367: f64, t471: f64, t2251: f64, t372: f64, t5296: f64, t5297: f64, t5405: f64, t17350: f64, t3767: f64, t1121: f64, t1248: f64, t606: f64, t3604: f64, t17353: f64, t5277: f64, t3630: f64, t12784: f64, t12866: f64, t12910: f64, t17619: f64, t17622: f64, t17625: f64, t17629: f64, t3625: f64, t5402: f64, t5056: f64, t12803: f64, t1715: f64, t12810: f64, t3603: f64, t3362: f64, t12787: f64, t1285: f64, t12865: f64, t5302: f64, t4181: f64, t13396: f64, t1042: f64, t3588: f64, t5332: f64, t3720: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t17635, t17640, t17646, t17649, t17650) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1434(t17633, t3629, t3626, t2258, t3628, t5351, t3367, t471, t2251, t372, t5296, t5297, t5405);
        let (t17651, t17654, t17658, t17662) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1435(t17649, t17650, t17350, t3767, t1121, t1248, t606, t3604, t17353, t372, t5277, t3630);
        let t17665 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1436(t12784, t12866, t12910, t17619, t17622, t17625, t17629, t17635, t17640, t17646, t17651, t17654, t17658, t17662, t3625, t5402);
        let (t17669, t17674, t17679, t17684, t17690) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1437(t5056, t5405, t3626, t12803, t471, t1715, t12810, t3603, t3362, t2251, t5351, t12787);
        let (t17693, t17695, t17696, t17700, t17705) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1438(t1285, t12865, t372, t5302, t4181, t5405, t13396, t1042, t3588, t3603, t5332, t3720);
    (t17665, t17669, t17674, t17679, t17684, t17690, t17693, t17695, t17696, t17700, t17705)
}
