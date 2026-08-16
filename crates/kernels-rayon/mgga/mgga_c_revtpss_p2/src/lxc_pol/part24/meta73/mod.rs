//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta73 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk454;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk455;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk456;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk457;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk458;
use chunk5::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk459;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta73(t1892: f64, t225: f64, t561: f64, t1437: f64, t1883: f64, t546: f64, t1431: f64, t1436: f64, t213: f64, t820: f64, t1427: f64, t1361: f64, t1366: f64, t1424: f64, t1319: f64, t1322: f64, t1334: f64, t1339: f64, t1342: f64, t1343: f64, t1450: f64, t1858: f64, t1860: f64, t1868: f64, t198: f64, t532: f64, t679: f64, t704: f64, t118: f64, t1502: f64, t1519: f64, t1843: f64, t1847: f64, t508: f64, t511: f64, t569: f64, t651: f64, t3: f64, t117: f64, t1518: f64, param_d: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t1893 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk454(t1892, t225);
        let (t1894, t1897, t1900, t1903) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk455(t1893, t561, t1437, t1883, t1892, t546, t1431, t1436, t213, t820);
        let t1904 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk456(t1427, t1903);
        let t1907 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk457(t1361, t1366, t1424, t1894, t1904, t213);
        let t1911 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk458(t1319, t1322, t1334, t1339, t1342, t1343, t1450, t1858, t1860, t1868, t1907, t198, t532, t679, t704);
        let (t1913, t1914, t1916, t1918) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk459(t118, t1502, t1519, t1843, t1847, t1911, t508, t511, t569, t651, t3, t117, t1518, param_d);
    (t1893, t1894, t1897, t1900, t1903, t1904, t1907, t1911, t1913, t1914, t1916, t1918)
}
