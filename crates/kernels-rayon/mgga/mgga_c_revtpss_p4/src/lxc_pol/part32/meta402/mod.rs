//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta402 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1386;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1387;
use chunk2::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1388;
use chunk3::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1389;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta402(t18281: f64, t190: f64, t706: f64, t14441: f64, t10593: f64, t10597: f64, t189: f64, t5819: f64, t606: f64, t14330: f64, t10608: f64, t4308: f64, t4311: f64, t10613: f64, t10592: f64, t10596: f64, t10604: f64, t10611: f64, t14433: f64, t14618: f64, t9524: f64, t9542: f64, t18534: f64, t18553: f64, t18568: f64, t225: f64, t1553: f64, t73: f64, t2475: f64, t5966: f64, t775: f64, t4343: f64, t4416: f64, t5962: f64, t853: f64, t18392: f64, t832: f64, t1555: f64, t227: f64, t229: f64, t4409: f64, t4415: f64, t4417: f64, t4420: f64, t6006: f64, t6010: f64, t6013: f64, t830: f64, t833: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t18571, t18572, t18573, t18574, t18578, t18579, t18581) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1386(t18281, t190, t706, t14441, t10593, t10597, t189, t5819, t606, t14330, t10608, t4308, t4311);
        let (t18582, t18583) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1387(t10613, t10592, t10596, t10604, t10611, t14433, t14618, t18571, t18572, t18573, t18574, t18578, t18579, t18581, t9524, t9542);
        let (t18586, t18592, t18600, t18603) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1388(t18534, t18553, t18568, t18583, t225, t1553, t73, t2475, t5966, t775, t4343, t4416);
        let t18615 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1389(t5962, t853, t775, t18392, t832, t1553, t1555, t18586, t18592, t18600, t18603, t227, t229, t4409, t4415, t4417, t4420, t6006, t6010, t6013, t830, t833);
    (t18571, t18572, t18573, t18574, t18578, t18579, t18581, t18582, t18615)
}
