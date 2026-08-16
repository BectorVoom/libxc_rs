//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta90 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk552;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk553;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk554;
use chunk3::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk555;
use chunk4::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk556;
use chunk5::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk557;
use chunk6::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk558;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta90(t1274: f64, t1276: f64, t1288: f64, t1293: f64, t1296: f64, t1297: f64, t1390: f64, t1789: f64, t1791: f64, t1799: f64, t1845: f64, t193: f64, t533: f64, t680: f64, t705: f64, t113: f64, t1442: f64, t1459: f64, t1774: f64, t1778: f64, t510: f64, t513: f64, t574: f64, t652: f64, t3: f64, t1401: f64, t1458: f64, t577: f64, t33: f64, t605: f64, t71: f64, t79: f64, t107: f64, t63: f64, t202: f64, t154: f64, t204: f64, t209: f64, t220: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t1849 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk552(t1274, t1276, t1288, t1293, t1296, t1297, t1390, t1789, t1791, t1799, t1845, t193, t533, t680, t705);
        let t1851 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk553(t113, t1442, t1459, t1774, t1778, t1849, t510, t513, t574, t652);
        let (t1852, t1858, t1860) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk554(t1851, t3, t1401, t1458, t577, t33, t605);
        let t1864 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk555(t71, t79);
        let (t1871, t1877) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk556(t107, t63, t193, t202);
        let t1878 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk557(t154, t204);
        let (t1879, t1880) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk558(t209, t220, t1878);
    (t1849, t1851, t1852, t1858, t1860, t1864, t1871, t1877, t1878, t1879, t1880)
}
