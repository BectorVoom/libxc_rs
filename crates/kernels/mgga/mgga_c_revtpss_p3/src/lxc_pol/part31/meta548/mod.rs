//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta548 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1941;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1942;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1943;
use chunk3::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1944;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta548<F: Float>(t1089: F, t29759: F, t1972: F, t6317: F, t1675: F, t25538: F, t27448: F, t27460: F, t27462: F, t27471: F, t27489: F, t375: F, t6285: F, t6289: F, t6293: F, t6323: F, t6327: F, t7111: F, t7132: F, t1665: F, t1671: F, t25500: F, t25505: F, t25509: F, t25517: F, t25522: F, t25560: F, t25580: F, t27450: F, t27479: F, t27539: F, t6263: F, t6268: F, t6273: F, t6278: F, t6302: F, t6308: F, t6312: F, t6331: F, t6339: F, t7117: F, t7122: F, t225: F, t385: F, t1982: F, t6343: F, t1695: F, t7821: F, t7160: F, t1668: F, t27604: F, t6299: F, t7168: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t29760, t29779, t29782) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1941::<F>(t1089, t29759, t1972, t6317, t1675, t25538, t27448, t27460, t27462, t27471, t27489, t375, t6285, t6289, t6293, t6323, t6327, t7111, t7132);
        let t29806 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1942::<F>(t1665, t1671, t25500, t25505, t25509, t25517, t25522, t25560, t25580, t27450, t27479, t27539, t6263, t6268, t6273, t6278, t6302, t6308, t6312, t6331, t6339, t7117, t7122, t7132);
        let t29807 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1943::<F>(t29782, t29806);
        let (t29809, t29812, t29817, t29818, t29822, t29826) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1944::<F>(t225, t29807, t385, t1982, t6343, t1695, t7821, t7160, t1089, t1668, t27604, t6299, t7168);
    (t29760, t29779, t29807, t29809, t29812, t29817, t29818, t29822, t29826)
}
