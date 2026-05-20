//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta535 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2067;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2068;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2069;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2070;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta535<F: Float>(t21829: F, t665: F, t10227: F, t5895: F, t658: F, t1504: F, t2: F, t580: F, t2349: F, t5823: F, t9342: F, t100: F, t10241: F, t5907: F, t661: F, t1509: F, t2357: F, t5911: F, t108: F, t105: F, t13475: F, t13496: F, t1507: F, t4280: F, t4284: F, t5896: F, t5899: F, t5902: F, t656: F, t662: F, t97: F, t655: F, t10201: F, t10202: F, t13448: F, t13451: F, t13453: F, t21818: F, t21821: F, t21824: F, t21827: F, t69: F, t114: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t21830, t21836, t21840, t21846, t21850, t21851) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2067::<F>(t21829, t665, t10227, t5895, t658, t1504, t2, t580, t2349, t5823, t9342, t100);
        let (t21861, t21865, t21869, t21872, t21873, t21876) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2068::<F>(t10241, t5907, t661, t1509, t2, t580, t2357, t5911, t21850, t108, t105, t13475, t13496, t1507, t21836, t21840, t21846, t21851, t4280, t4284, t5896, t5899, t5902, t656, t662, t97);
        let (t21877, t21880) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2069::<F>(t21876, t655, t10201, t10202, t13448, t13451, t13453, t21818, t21821, t21824, t21827, t21830, t69);
        let t21881 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2070::<F>(t114, t21880);
    (t21830, t21850, t21861, t21865, t21869, t21872, t21873, t21876, t21877, t21881)
}
