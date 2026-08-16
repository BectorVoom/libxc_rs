//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta171 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1025;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1026;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1027;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1028;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1029;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1030;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta171<F: Float>(t366: F, t4857: F, t4589: F, t4592: F, t4594: F, t4597: F, t4634: F, t4638: F, t4716: F, t4718: F, t4721: F, t4723: F, t4727: F, t4731: F, t4736: F, t1045: F, t373: F, t1042: F, t1065: F, t905: F, t1469: F, t999: F, t1032: F, t1647: F, t1040: F, t1025: F, t1028: F, t1041: F, t1047: F, t1665: F, t1671: F, t3124: F, t3127: F, t3194: F, t3203: F, t3211: F, t3216: F, t3224: F, t4854: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let t4858 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1025::<F>(t366, t4857);
        let t4866 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1026::<F>(t4589, t4592, t4594, t4597, t4634, t4638, t4716, t4718, t4721, t4723, t4727, t4731, t4736);
        let (t4868, t4869, t4872) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1027::<F>(t1045, t373, t4866, t1042, t1065, t905);
        let t4873 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1028::<F>(t1469, t999);
        let (t4874, t4875, t4878, t4879) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1029::<F>(t4872, t4873, t1042, t1032, t1647, t1040);
        let t4883 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1030::<F>(t1025, t1028, t1041, t1047, t1665, t1671, t3124, t3127, t3194, t3203, t3211, t3216, t3224, t4854, t4858, t4869, t4875, t4879);
    (t4858, t4866, t4868, t4869, t4872, t4873, t4874, t4875, t4878, t4879, t4883)
}
