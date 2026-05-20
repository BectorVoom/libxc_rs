//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta415 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1481;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1482;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1483;
use chunk3::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1484;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta415<F: Float>(t18281: F, t190: F, t706: F, t14441: F, t10593: F, t10597: F, t189: F, t5819: F, t606: F, t14330: F, t10608: F, t4308: F, t4311: F, t10613: F, t10592: F, t10596: F, t10604: F, t10611: F, t14433: F, t14618: F, t9524: F, t9542: F, t18534: F, t18553: F, t18568: F, t225: F, t1553: F, t73: F, t2475: F, t5966: F, t775: F, t4343: F, t4416: F, t5962: F, t853: F, t18392: F, t832: F, t1555: F, t227: F, t229: F, t4409: F, t4415: F, t4417: F, t4420: F, t6006: F, t6010: F, t6013: F, t830: F, t833: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t18571, t18572, t18573, t18574, t18578, t18579, t18581) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1481::<F>(t18281, t190, t706, t14441, t10593, t10597, t189, t5819, t606, t14330, t10608, t4308, t4311);
        let (t18582, t18583) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1482::<F>(t10613, t10592, t10596, t10604, t10611, t14433, t14618, t18571, t18572, t18573, t18574, t18578, t18579, t18581, t9524, t9542);
        let (t18586, t18592, t18600, t18603) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1483::<F>(t18534, t18553, t18568, t18583, t225, t1553, t73, t2475, t5966, t775, t4343, t4416);
        let t18615 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1484::<F>(t5962, t853, t775, t18392, t832, t1553, t1555, t18586, t18592, t18600, t18603, t227, t229, t4409, t4415, t4417, t4420, t6006, t6010, t6013, t830, t833);
    (t18571, t18572, t18573, t18574, t18578, t18579, t18581, t18582, t18615)
}
