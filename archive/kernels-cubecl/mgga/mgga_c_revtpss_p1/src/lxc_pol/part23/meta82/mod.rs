//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta82 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk568;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk569;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk570;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk571;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk572;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk573;
use chunk6::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk574;
use chunk7::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk575;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta82<F: Float>(t1132: F, t1723: F, t1139: F, t1145: F, t1715: F, t141: F, t1137: F, t1144: F, t1717: F, t1150: F, t1131: F, t1154: F, t1163: F, t1166: F, t1169: F, t1173: F, t448: F, t1182: F, t1185: F, t1188: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t1724, t1727, t1729, t1730, t1732) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk568::<F>(t1132, t1723, t1139, t1145, t1715, t141, t1137, t1144, t1717);
        let t1733 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk569::<F>(t1150, t1732);
        let (t1735, t1737) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk570::<F>(t1131, t1733, t1154, t1717);
        let t1744 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk571::<F>(t1163, t1166, t1717, t1724, t1727, t1730);
        let t1745 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk572::<F>(t1169, t1744);
        let t1749 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk573::<F>(t1173, t1717);
        let (t1750, t1756) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk574::<F>(t1749, t448, t1182, t1185, t1717, t1724, t1727, t1730);
        let t1757 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk575::<F>(t1188, t1756);
    (t1729, t1732, t1733, t1735, t1737, t1744, t1745, t1749, t1750, t1756, t1757)
}
