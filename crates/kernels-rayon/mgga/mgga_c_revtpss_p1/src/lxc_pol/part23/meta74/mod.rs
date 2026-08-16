//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta74 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk513;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk514;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk515;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk516;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk517;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk518;
use chunk6::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk519;
use chunk7::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk520;
use chunk8::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk521;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta74(t1471: f64, t1487: f64, t1494: f64, t71: f64, t85: f64, t5: f64, t1466: f64, t603: f64, t91: f64, t117: f64, t1468: f64, t100: f64, t55: f64, tau1: f64, t108: f64, t105: f64, t109: f64, t97: f64, t114: f64, t655: f64, t653: f64, t69: f64, t508: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t1497 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk513(t1471, t1487, t1494, t71, t85);
        let t1501 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk514(t5, t1466, t1497, t603, t91);
        let t1502 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk515(t117, t1501);
        let t1504 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk516(t1468);
        let (t1505, t1507) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk517(t100, t1504, t55, tau1);
        let t1509 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk518(t1504);
        let (t1510, t1513) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk519(t108, t1509, t105, t109, t1505, t1507, t97);
        let (t1514, t1518) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk520(t114, t1513, t655, t653, t69);
        let t1519 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk521(t1518, t508);
    (t1497, t1501, t1502, t1504, t1505, t1507, t1509, t1510, t1513, t1514, t1518, t1519)
}
