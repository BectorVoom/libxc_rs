//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta88 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk634;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk635;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk636;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk637;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk638;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk639;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk640;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta88<F: Float>(t1394: F, t1868: F, t1877: F, t539: F, t541: F, t543: F, t828: F, t1390: F, t1414: F, t1368: F, t1370: F, t1378: F, t1383: F, t1388: F, t1407: F, t1410: F, t1873: F, t225: F, t561: F, t1437: F, t546: F, t1431: F, t1436: F, t213: F, t820: F, t1427: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t1879 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk634::<F>(t1394, t1868);
        let t1882 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk635::<F>(t1877, t1879, t539, t541);
        let t1883 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk636::<F>(t1882, t543);
        let (t1885, t1889, t1892) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk637::<F>(t1883, t828, t1390, t1414, t1868, t1368, t1370, t1378, t1383, t1388, t1407, t1410, t1873);
        let t1893 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk638::<F>(t1892, t225);
        let (t1894, t1897, t1900, t1903) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk639::<F>(t1893, t561, t1437, t1883, t1892, t546, t1431, t1436, t213, t820);
        let t1904 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk640::<F>(t1427, t1903);
    (t1879, t1882, t1883, t1885, t1889, t1892, t1893, t1894, t1897, t1900, t1903, t1904)
}
