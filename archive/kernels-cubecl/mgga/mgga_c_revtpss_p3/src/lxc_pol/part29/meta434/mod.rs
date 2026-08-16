//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta434 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1611;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1612;
use chunk2::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1613;
use chunk3::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1614;
use chunk4::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1615;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta434<F: Float>(t17633: F, t3629: F, t3626: F, t2258: F, t3628: F, t5351: F, t3367: F, t471: F, t2251: F, t372: F, t5296: F, t5297: F, t5405: F, t17350: F, t3767: F, t1121: F, t1248: F, t606: F, t3604: F, t17353: F, t5277: F, t3630: F, t12784: F, t12866: F, t12910: F, t17619: F, t17622: F, t17625: F, t17629: F, t3625: F, t5402: F, t5056: F, t12803: F, t1715: F, t12810: F, t3603: F, t3362: F, t12787: F, t1285: F, t12865: F, t5302: F, t4181: F, t13396: F, t1042: F, t3588: F, t5332: F, t3720: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t17635, t17640, t17646, t17649, t17650) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1611::<F>(t17633, t3629, t3626, t2258, t3628, t5351, t3367, t471, t2251, t372, t5296, t5297, t5405);
        let (t17651, t17654, t17658, t17662) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1612::<F>(t17649, t17650, t17350, t3767, t1121, t1248, t606, t3604, t17353, t372, t5277, t3630);
        let t17665 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1613::<F>(t12784, t12866, t12910, t17619, t17622, t17625, t17629, t17635, t17640, t17646, t17651, t17654, t17658, t17662, t3625, t5402);
        let (t17669, t17674, t17679, t17684, t17690) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1614::<F>(t5056, t5405, t3626, t12803, t471, t1715, t12810, t3603, t3362, t2251, t5351, t12787);
        let (t17693, t17695, t17696, t17700, t17705) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1615::<F>(t1285, t12865, t372, t5302, t4181, t5405, t13396, t1042, t3588, t3603, t5332, t3720);
    (t17665, t17669, t17674, t17679, t17684, t17690, t17693, t17695, t17696, t17700, t17705)
}
