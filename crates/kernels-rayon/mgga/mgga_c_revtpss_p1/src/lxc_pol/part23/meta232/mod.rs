//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta232 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1361;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1362;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1363;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1364;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1365;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1366;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta232(t1250: f64, t6688: f64, t3720: f64, t1222: f64, t1261: f64, t1782: f64, t1808: f64, t3657: f64, t3684: f64, t3718: f64, t464: f64, t5358: f64, t5363: f64, t5366: f64, t5373: f64, t5379: f64, t5381: f64, t5391: f64, t6653: f64, t6659: f64, t6663: f64, t6667: f64, t6673: f64, t6679: f64, t6683: f64, t6651: f64, t225: f64, t494: f64, t1828: f64, t3737: f64, t1280: f64, t6573: f64, t1287: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t6689, t6690) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1361(t1250, t6688, t3720);
        let t6694 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1362(t1222, t1261, t1782, t1808, t3657, t3684, t3718, t464, t5358, t5363, t5366, t5373, t5379, t5381, t5391, t6653, t6659, t6663, t6667, t6673, t6679, t6683, t6690);
        let t6695 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1363(t6651, t6694);
        let (t6697, t6702) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1364(t225, t494, t6695, t1828);
        let t6703 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1365(t3737, t6702);
        let (t6714, t6717) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1366(t1280, t6573, t1287, t6688);
    (t6689, t6690, t6695, t6697, t6702, t6703, t6714, t6717)
}
