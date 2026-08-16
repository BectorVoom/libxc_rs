//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta228 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1339;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1340;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1341;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1342;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1343;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1344;
use chunk6::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1345;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta228(t3523: f64, t6555: f64, t1196: f64, t3546: f64, t5044: f64, t6423: f64, t6427: f64, t6431: f64, t459: f64, t1774: f64, t1211: f64, t1828: f64, t1277: f64, t3579: f64, t1477: f64, t476: f64, t52: f64, t475: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t6556, t6558, t6563, t6564) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1339(t3523, t6555, t1196, t3546, t5044, t6423, t6427, t6431, t459);
        let t6573 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1340(t1774);
        let t6574 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1341(t1211, t6573);
        let t6580 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1342(t1774, t1828, t1277);
        let t6587 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1343(t3579, t5044, t6423, t6427, t6431);
        let t6588 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1344(t1211, t6587);
        let (t6593, t6594) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1345(t1477, t476, t52, t475);
    (t6556, t6558, t6563, t6564, t6573, t6574, t6580, t6587, t6588, t6593, t6594)
}
