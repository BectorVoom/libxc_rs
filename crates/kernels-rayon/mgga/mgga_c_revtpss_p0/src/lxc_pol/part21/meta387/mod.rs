//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta387 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1821;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1822;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1823;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1824;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1825;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1826;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta387(t1211: f64, t12646: f64, t1214: f64, t3790: f64, t1277: f64, t3552: f64, t487: f64, t1208: f64, t3551: f64, t1210: f64, t1215: f64, t12600: f64, t12603: f64, t12607: f64, t12622: f64, t12628: f64, t12630: f64, t12633: f64, t12641: f64, t1295: f64, t3556: f64, t3567: f64, t3569: f64, t3572: f64, t3576: f64, t3585: f64, t3732: f64, t3791: f64, t1209: f64, t3727: f64, t460: f64, t12295: f64, t12292: f64, t12297: f64, t12299: f64, t12301: f64, t12303: f64, t12307: f64, t12310: f64, t12314: f64, t12317: f64, t12320: f64, t459: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t12647, t12650, t12651, t12654, t12657) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1821(t1211, t12646, t1214, t3790, t1277, t3552, t487, t1208, t3551);
        let t12658 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1822(t12657, t487);
        let t12663 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1823(t1210, t1215, t12600, t12603, t12607, t12622, t12628, t12630, t12633, t12641, t12647, t12651, t12654, t12658, t1295, t3556, t3567, t3569, t3572, t3576, t3585, t3732, t3791);
        let t12666 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1824(t1209, t3727);
        let (t12673, t12678, t12689) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1825(t3727, t460, t12295, t12292, t12297, t12299, t12301, t12303, t12307, t12310, t12314, t12317, t12320);
        let t12690 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1826(t12689, t459);
    (t12647, t12650, t12651, t12654, t12657, t12658, t12663, t12666, t12673, t12678, t12689, t12690)
}
