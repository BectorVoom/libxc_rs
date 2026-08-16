//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta324 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1614;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1615;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1616;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1617;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta324(t1868: f64, t4010: f64, t1353: f64, t13767: f64, t2661: f64, t550: f64, t5658: f64, t543: f64, t3992: f64, t5610: f64, t9775: f64, t1889: f64, t9779: f64, t828: f64, t9954: f64, t1398: f64, t3935: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t13768, t13769, t13770, t13772, t13775, t13776, t13778, t13779, t13781) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1614(t1868, t4010, t1353, t13767, t2661, t550, t5658, t543, t3992, t5610, t9775, t1889, t9779);
        let t13783 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1615(t828, t9954);
        let t13784 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1616(t1398, t1868);
        let t13789 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1617(t3935, t828);
    (t13768, t13769, t13770, t13772, t13775, t13776, t13778, t13779, t13781, t13783, t13784, t13789)
}
