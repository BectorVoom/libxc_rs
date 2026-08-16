//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta87 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk541;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk542;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk543;
use chunk3::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk544;
use chunk4::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk545;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta87(t25: f64, t28: f64, t17: f64, t1788: f64, t1787: f64, t182: f64, t1298: f64, t1408: f64, t1302: f64, t1649: f64, zeta_threshold: f64, t210: f64, t214: f64, t1313: f64, t1315: f64, t1322: f64, t562: f64, t119: f64, t225: f64, t554: f64, t1274: f64, t1276: f64, t1288: f64, t1293: f64, t1296: f64, t680: f64, t705: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t1789, t1791, t1799) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk541(t25, t28, t17, t1788, t1787, t182, t1298, t1408, t1302, t1649, zeta_threshold);
        let (t1804, t1807) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk542(t1799, t210, t214, t1313, t1315, t1322);
        let (t1808, t1810, t1811) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk543(t1807, t562, t119, t1799, t210);
        let t1814 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk544(t1807, t225);
        let (t1815, t1819) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk545(t1814, t554, t1274, t1276, t1288, t1293, t1296, t1789, t1791, t225, t680, t705);
    (t1789, t1791, t1799, t1804, t1807, t1808, t1810, t1811, t1814, t1815, t1819)
}
