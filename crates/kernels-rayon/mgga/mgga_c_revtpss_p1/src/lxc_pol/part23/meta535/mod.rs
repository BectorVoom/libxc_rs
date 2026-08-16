//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta535 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2067;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2068;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2069;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2070;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta535(t21829: f64, t665: f64, t10227: f64, t5895: f64, t658: f64, t1504: f64, t2: f64, t580: f64, t2349: f64, t5823: f64, t9342: f64, t100: f64, t10241: f64, t5907: f64, t661: f64, t1509: f64, t2357: f64, t5911: f64, t108: f64, t105: f64, t13475: f64, t13496: f64, t1507: f64, t4280: f64, t4284: f64, t5896: f64, t5899: f64, t5902: f64, t656: f64, t662: f64, t97: f64, t655: f64, t10201: f64, t10202: f64, t13448: f64, t13451: f64, t13453: f64, t21818: f64, t21821: f64, t21824: f64, t21827: f64, t69: f64, t114: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t21830, t21836, t21840, t21846, t21850, t21851) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2067(t21829, t665, t10227, t5895, t658, t1504, t2, t580, t2349, t5823, t9342, t100);
        let (t21861, t21865, t21869, t21872, t21873, t21876) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2068(t10241, t5907, t661, t1509, t2, t580, t2357, t5911, t21850, t108, t105, t13475, t13496, t1507, t21836, t21840, t21846, t21851, t4280, t4284, t5896, t5899, t5902, t656, t662, t97);
        let (t21877, t21880) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2069(t21876, t655, t10201, t10202, t13448, t13451, t13453, t21818, t21821, t21824, t21827, t21830, t69);
        let t21881 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2070(t114, t21880);
    (t21830, t21850, t21861, t21865, t21869, t21872, t21873, t21876, t21877, t21881)
}
