//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta189 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1130;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1131;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1132;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1133;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1134;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1135;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta189(t5332: f64, t5341: f64, t3720: f64, t1248: f64, t1774: f64, t1250: f64, t1794: f64, t73: f64, t1214: f64, t471: f64, t140: f64, t1781: f64, t1222: f64, t127: f64, t1789: f64, t371: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t5342, t5343) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1130(t5332, t5341, t3720);
        let (t5346, t5347, t5348) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1131(t1248, t1774, t1250, t3720);
        let t5351 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1132(t1794, t73);
        let t5352 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1133(t1214, t471);
        let (t5353, t5354) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1134(t5351, t5352, t3720);
        let (t5357, t5358, t5362) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1135(t140, t1781, t1222, t127, t1789, t371);
    (t5342, t5343, t5346, t5347, t5348, t5351, t5352, t5353, t5354, t5357, t5358, t5362)
}
