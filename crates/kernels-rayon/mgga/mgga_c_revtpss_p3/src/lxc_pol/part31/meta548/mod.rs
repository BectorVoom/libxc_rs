//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta548 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1941;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1942;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1943;
use chunk3::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1944;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta548(t1089: f64, t29759: f64, t1972: f64, t6317: f64, t1675: f64, t25538: f64, t27448: f64, t27460: f64, t27462: f64, t27471: f64, t27489: f64, t375: f64, t6285: f64, t6289: f64, t6293: f64, t6323: f64, t6327: f64, t7111: f64, t7132: f64, t1665: f64, t1671: f64, t25500: f64, t25505: f64, t25509: f64, t25517: f64, t25522: f64, t25560: f64, t25580: f64, t27450: f64, t27479: f64, t27539: f64, t6263: f64, t6268: f64, t6273: f64, t6278: f64, t6302: f64, t6308: f64, t6312: f64, t6331: f64, t6339: f64, t7117: f64, t7122: f64, t225: f64, t385: f64, t1982: f64, t6343: f64, t1695: f64, t7821: f64, t7160: f64, t1668: f64, t27604: f64, t6299: f64, t7168: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t29760, t29779, t29782) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1941(t1089, t29759, t1972, t6317, t1675, t25538, t27448, t27460, t27462, t27471, t27489, t375, t6285, t6289, t6293, t6323, t6327, t7111, t7132);
        let t29806 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1942(t1665, t1671, t25500, t25505, t25509, t25517, t25522, t25560, t25580, t27450, t27479, t27539, t6263, t6268, t6273, t6278, t6302, t6308, t6312, t6331, t6339, t7117, t7122, t7132);
        let t29807 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1943(t29782, t29806);
        let (t29809, t29812, t29817, t29818, t29822, t29826) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1944(t225, t29807, t385, t1982, t6343, t1695, t7821, t7160, t1089, t1668, t27604, t6299, t7168);
    (t29760, t29779, t29807, t29809, t29812, t29817, t29818, t29822, t29826)
}
