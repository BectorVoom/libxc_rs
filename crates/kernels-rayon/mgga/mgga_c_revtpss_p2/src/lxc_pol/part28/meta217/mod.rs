//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta217 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1029;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1030;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1031;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1032;
use chunk4::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1033;
use chunk5::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1034;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta217(t225: f64, t4746: f64, t366: f64, t4589: f64, t4592: f64, t4594: f64, t4597: f64, t4634: f64, t4638: f64, t4716: f64, t4718: f64, t4721: f64, t4723: f64, t4727: f64, t4731: f64, t4736: f64, t1045: f64, t373: f64, t1042: f64, t1065: f64, t905: f64, t1469: f64, t999: f64, t1032: f64, t1647: f64, t1040: f64, t1025: f64, t1028: f64, t1041: f64, t1047: f64, t1665: f64, t1671: f64, t3124: f64, t3127: f64, t3194: f64, t3203: f64, t3211: f64, t3216: f64, t3224: f64, t4854: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t4857 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1029(t225, t4746);
        let (t4858, t4866) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1030(t366, t4857, t4589, t4592, t4594, t4597, t4634, t4638, t4716, t4718, t4721, t4723, t4727, t4731, t4736);
        let (t4868, t4869) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1031(t1045, t373, t4866, t1042);
        let t4872 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1032(t1065, t905);
        let (t4873, t4874, t4875) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1033(t1469, t999, t4872, t1042);
        let (t4878, t4879, t4883) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1034(t1032, t1647, t1040, t1025, t1028, t1041, t1047, t1665, t1671, t3124, t3127, t3194, t3203, t3211, t3216, t3224, t4854, t4858, t4869, t4875);
    (t4857, t4858, t4866, t4868, t4869, t4872, t4873, t4874, t4875, t4878, t4879, t4883)
}
