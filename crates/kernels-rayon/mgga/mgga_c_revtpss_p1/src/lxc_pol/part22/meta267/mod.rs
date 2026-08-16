//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta267 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1634;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1635;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1636;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1637;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1638;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1639;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta267(t187: f64, t6800: f64, t5636: f64, t2522: f64, t2562: f64, t2579: f64, t2587: f64, t3871: f64, t3873: f64, t4027: f64, t6780: f64, t6802: f64, t2569: f64, t3854: f64, t3859: f64, t3862: f64, t3865: f64, t3867: f64, t4035: f64, t4037: f64, t4042: f64, t6777: f64, t6778: f64, t6779: f64, t225: f64, t1868: f64, t4049: f64, t1394: f64, t6816: f64, t1877: f64, t1879: f64, t539: f64, t541: f64, t543: f64, t1390: f64, t828: f64, t124: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t6827, t6828, t6829) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1634(t187, t6800, t5636, t2522, t2562, t2579, t2587, t3871, t3873, t4027, t6780, t6802);
        let t6830 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1635(t2569, t3854, t3859, t3862, t3865, t3867, t4035, t4037, t4042, t6777, t6778, t6779);
        let (t6832, t6836) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1636(t225, t6829, t6830, t1868);
        let (t6837, t6840, t6843) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1637(t4049, t6836, t1394, t6816, t1877, t1879, t539, t541, t6832);
        let t6844 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1638(t543, t6843);
        let (t6846, t6849) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1639(t1390, t6844, t828, t124, t6836);
    (t6827, t6828, t6832, t6836, t6837, t6840, t6843, t6844, t6846, t6849)
}
