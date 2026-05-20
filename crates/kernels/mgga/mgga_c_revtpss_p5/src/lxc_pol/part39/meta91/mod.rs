//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta91 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk519;
use chunk1::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk520;
use chunk2::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk521;
use chunk3::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk522;
use chunk4::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk523;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta91<F: Float>(t1892: F, t225: F, t561: F, t1437: F, t1883: F, t546: F, t1431: F, t1436: F, t213: F, t820: F, t1427: F, t1361: F, t1366: F, t1424: F, t1319: F, t1322: F, t1334: F, t1339: F, t1342: F, t1343: F, t1450: F, t1858: F, t1860: F, t1868: F, t198: F, t532: F, t679: F, t704: F, t118: F, t1502: F, t1519: F, t1843: F, t1847: F, t508: F, t511: F, t569: F, t651: F, t3: F, param_d: F, t117: F, t1518: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t1893, t1894, t1903) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk519::<F>(t1892, t225, t561, t1437, t1883, t546, t1431, t1436, t213, t820);
        let t1904 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk520::<F>(t1427, t1903);
        let (t1907, t1911) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk521::<F>(t1361, t1366, t1424, t1894, t1904, t213, t1319, t1322, t1334, t1339, t1342, t1343, t1450, t1858, t1860, t1868, t198, t532, t679, t704);
        let (t1913, t1914, t1916) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk522::<F>(t118, t1502, t1519, t1843, t1847, t1911, t508, t511, t569, t651, t3, param_d);
        let t1918 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk523::<F>(t117, t1518);
    (t1893, t1903, t1904, t1907, t1911, t1913, t1914, t1916, t1918)
}
