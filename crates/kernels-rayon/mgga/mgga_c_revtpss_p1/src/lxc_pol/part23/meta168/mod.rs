//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta168 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1010;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1011;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1012;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1013;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1014;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1015;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta168(t1678: f64, t994: f64, t1668: f64, t73: f64, t3095: f64, t3092: f64, t3093: f64, t357: f64, t1592: f64, t1058: f64, t1660: f64, t1053: f64, t1659: f64, t225: f64, t4743: f64, t366: f64, t1065: f64, t2857: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t4778 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1010(t1678, t994);
        let t4781 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1011(t1668, t73);
        let t4782 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1012(t3095, t4781);
        let (t4783, t4786) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1013(t3092, t4782, t3093, t357);
        let t4787 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1014(t1592, t4786);
        let (t4788, t4792, t4794, t4797, t4798, t4801) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1015(t3092, t4787, t1058, t1660, t1053, t1659, t225, t4743, t366, t1065, t2857);
    (t4778, t4781, t4782, t4783, t4786, t4787, t4788, t4792, t4794, t4797, t4798, t4801)
}
