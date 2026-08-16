//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta86 (260520-c91 hierarchical CSE).
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
mod chunk9;
mod chunk10;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk616;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk617;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk618;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk619;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk620;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk621;
use chunk6::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk622;
use chunk7::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk623;
use chunk8::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk624;
use chunk9::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk625;
use chunk10::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk626;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta86(t1819: f64, t1821: f64, t546: f64, t548: f64, t550: f64, t1343: f64, t820: f64, t1367: f64, t1799: f64, t1315: f64, t1327: f64, t1341: f64, t1360: f64, t1363: f64, t1811: f64, t1815: f64, t559: f64, t539: f64, t1380: f64, t553: f64, t1336: f64, t1814: f64, t544: f64, t564: f64, t1378: f64, t1375: f64, t1808: f64, t568: f64, t1274: f64, t1276: f64, t1288: f64, t1293: f64, t1296: f64, t1297: f64, t1390: f64, t1789: f64, t1791: f64, t193: f64, t533: f64, t680: f64, t705: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t1824 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk616(t1819, t1821, t546, t548);
        let t1825 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk617(t1824, t550);
        let t1827 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk618(t1343, t1825, t820);
        let t1831 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk619(t1367, t1799, t820);
        let t1834 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk620(t1315, t1327, t1341, t1360, t1363, t1811, t1815, t1827, t1831, t559);
        let (t1835, t1838) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk621(t1834, t539, t1380, t1825);
        let t1840 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk622(t1834, t553);
        let t1842 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk623(t1336, t1814, t1838, t1840, t544, t564);
        let t1843 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk624(t1378, t1842);
        let t1845 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk625(t1375, t1808, t1835, t1843, t568);
        let t1849 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk626(t1274, t1276, t1288, t1293, t1296, t1297, t1390, t1789, t1791, t1799, t1845, t193, t533, t680, t705);
    (t1824, t1825, t1827, t1831, t1834, t1835, t1838, t1840, t1842, t1843, t1845, t1849)
}
