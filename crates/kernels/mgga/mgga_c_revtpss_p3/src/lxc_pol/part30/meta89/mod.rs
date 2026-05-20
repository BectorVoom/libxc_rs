//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta89 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk561;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk562;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk563;
use chunk3::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk564;
use chunk4::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk565;
use chunk5::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk566;
use chunk6::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk567;
use chunk7::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk568;
use chunk8::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk569;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta89<F: Float>(t1883: F, t828: F, t1390: F, t1414: F, t1868: F, t1368: F, t1370: F, t1378: F, t1383: F, t1388: F, t1407: F, t1410: F, t1873: F, t225: F, t561: F, t1437: F, t546: F, t1431: F, t1436: F, t213: F, t820: F, t1427: F, t1361: F, t1366: F, t1424: F, t1319: F, t1322: F, t1334: F, t1339: F, t1342: F, t1343: F, t1450: F, t1858: F, t1860: F, t198: F, t532: F, t679: F, t704: F, t118: F, t1502: F, t1519: F, t1843: F, t1847: F, t508: F, t511: F, t569: F, t651: F, t3: F, param_d: F, t117: F, t1518: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t1885 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk561::<F>(t1883, t828, t1390);
        let t1889 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk562::<F>(t1414, t1868, t828);
        let t1892 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk563::<F>(t1368, t1370, t1378, t1383, t1388, t1407, t1410, t1873, t1885, t1889);
        let (t1893, t1894, t1903) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk564::<F>(t1892, t225, t561, t1437, t1883, t546, t1431, t1436, t213, t820);
        let t1904 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk565::<F>(t1427, t1903);
        let t1907 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk566::<F>(t1361, t1366, t1424, t1894, t1904, t213);
        let t1911 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk567::<F>(t1319, t1322, t1334, t1339, t1342, t1343, t1450, t1858, t1860, t1868, t1907, t198, t532, t679, t704);
        let (t1913, t1914, t1916) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk568::<F>(t118, t1502, t1519, t1843, t1847, t1911, t508, t511, t569, t651, t3, param_d);
        let t1918 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk569::<F>(t117, t1518);
    (t1885, t1889, t1892, t1893, t1903, t1904, t1907, t1911, t1913, t1914, t1916, t1918)
}
