//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta230 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1351;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1352;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1353;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1354;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1355;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1356;
use chunk6::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1357;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta230(t482: f64, t6628: f64, t3604: f64, t1042: f64, t3611: f64, t1469: f64, t3628: f64, t5351: f64, t3626: f64, t6587: f64, t371: f64, t372: f64, t1235: f64, t1247: f64, t1791: f64, t1797: f64, t3600: f64, t3610: f64, t3625: f64, t3671: f64, t3711: f64, t484: f64, t5254: f64, t5256: f64, t5266: f64, t5274: f64, t5293: f64, t5323: f64, t5327: f64, t6595: f64, t6598: f64, t6602: f64, t6611: f64, t6619: f64, t6625: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t6629, t6630, t6631) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1351(t482, t6628, t3604, t1042);
        let (t6634, t6635) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1352(t3611, t6629, t1042);
        let t6638 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1353(t1469, t3628);
        let t6639 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1354(t5351, t6638);
        let t6640 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1355(t3626, t6639);
        let (t6645, t6647) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1356(t482, t6587, t371, t372);
        let t6651 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1357(t1235, t1247, t1791, t1797, t3600, t3610, t3625, t3671, t3711, t484, t5254, t5256, t5266, t5274, t5293, t5323, t5327, t6595, t6598, t6602, t6611, t6619, t6625, t6631, t6635, t6640, t6647);
    (t6630, t6631, t6634, t6635, t6638, t6639, t6640, t6645, t6647, t6651)
}
