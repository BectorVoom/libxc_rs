//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta88 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk634;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk635;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk636;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk637;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk638;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk639;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk640;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta88(t1394: f64, t1868: f64, t1877: f64, t539: f64, t541: f64, t543: f64, t828: f64, t1390: f64, t1414: f64, t1368: f64, t1370: f64, t1378: f64, t1383: f64, t1388: f64, t1407: f64, t1410: f64, t1873: f64, t225: f64, t561: f64, t1437: f64, t546: f64, t1431: f64, t1436: f64, t213: f64, t820: f64, t1427: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t1879 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk634(t1394, t1868);
        let t1882 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk635(t1877, t1879, t539, t541);
        let t1883 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk636(t1882, t543);
        let (t1885, t1889, t1892) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk637(t1883, t828, t1390, t1414, t1868, t1368, t1370, t1378, t1383, t1388, t1407, t1410, t1873);
        let t1893 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk638(t1892, t225);
        let (t1894, t1897, t1900, t1903) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk639(t1893, t561, t1437, t1883, t1892, t546, t1431, t1436, t213, t820);
        let t1904 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk640(t1427, t1903);
    (t1879, t1882, t1883, t1885, t1889, t1892, t1893, t1894, t1897, t1900, t1903, t1904)
}
