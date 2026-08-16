//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta246 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1092;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1093;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1094;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta246(t1235: f64, t1247: f64, t1791: f64, t1797: f64, t3600: f64, t3610: f64, t3625: f64, t3671: f64, t3711: f64, t484: f64, t5254: f64, t5256: f64, t5266: f64, t5274: f64, t5293: f64, t5323: f64, t5327: f64, t6595: f64, t6598: f64, t6602: f64, t6611: f64, t6619: f64, t6625: f64, t6631: f64, t6635: f64, t6640: f64, t6647: f64, t3699: f64, t5819: f64, t1012: f64, t1225: f64, t5825: f64, t3692: f64, t344: f64, t5843: f64, t3618: f64, t6421: f64, t247: f64, t1264: f64, t6429: f64, t6425: f64, t1774: f64, t1794: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t6651 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1092(t1235, t1247, t1791, t1797, t3600, t3610, t3625, t3671, t3711, t484, t5254, t5256, t5266, t5274, t5293, t5323, t5327, t6595, t6598, t6602, t6611, t6619, t6625, t6631, t6635, t6640, t6647);
        let (t6652, t6653, t6658, t6659, t6662, t6663, t6667, t6673, t6678) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1093(t3699, t5819, t1012, t1225, t5825, t3692, t344, t5843, t3618, t6421, t247, t1264, t6429);
        let (t6679, t6683, t6688) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1094(t247, t6678, t1264, t6425, t1774, t1794);
    (t6651, t6652, t6653, t6658, t6659, t6662, t6663, t6667, t6673, t6679, t6683, t6688)
}
