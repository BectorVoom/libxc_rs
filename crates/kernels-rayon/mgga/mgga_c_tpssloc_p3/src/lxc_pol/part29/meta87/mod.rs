//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta87 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk567;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk568;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk569;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk570;
use chunk4::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk571;
use chunk5::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk572;
use chunk6::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk573;
use chunk7::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk574;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta87(t1807: f64, t562: f64, t119: f64, t1799: f64, t210: f64, t225: f64, t554: f64, t1274: f64, t1276: f64, t1288: f64, t1293: f64, t1296: f64, t1789: f64, t1791: f64, t680: f64, t705: f64, t1347: f64, t546: f64, t548: f64, t550: f64, t1343: f64, t820: f64, t1367: f64, t1315: f64, t1327: f64, t1341: f64, t1360: f64, t1363: f64, t559: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t1808, t1810, t1811) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk567(t1807, t562, t119, t1799, t210);
        let t1814 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk568(t1807, t225);
        let (t1815, t1819) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk569(t1814, t554, t1274, t1276, t1288, t1293, t1296, t1789, t1791, t225, t680, t705);
        let (t1821, t1824) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk570(t1347, t1799, t1819, t546, t548);
        let t1825 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk571(t1824, t550);
        let t1827 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk572(t1343, t1825, t820);
        let t1831 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk573(t1367, t1799, t820);
        let t1834 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk574(t1315, t1327, t1341, t1360, t1363, t1811, t1815, t1827, t1831, t559);
    (t1808, t1810, t1811, t1814, t1815, t1819, t1821, t1824, t1825, t1827, t1831, t1834)
}
