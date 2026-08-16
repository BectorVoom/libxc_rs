//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta278 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1263;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1264;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1265;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1266;
use chunk4::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1267;
use chunk5::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1268;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta278(t1052: f64, t1635: f64, t1920: f64, t1956: f64, t388: f64, t4557: f64, t4660: f64, t6685: f64, t6687: f64, t6771: f64, t7554: f64, t7557: f64, t7562: f64, t7566: f64, t7569: f64, t7594: f64, t7600: f64, t7625: f64, t265: f64, t394: f64, t1484: f64, t1915: f64, t202: f64, t7540: f64, t1530: f64, t1877: f64, t193: f64, t2522: f64, t6670: f64, t870: f64, t1070: f64, t1637: f64, t336: f64, t4700: f64, t6822: f64, t25: f64, t1409: f64, t1965: f64, t40: f64, t7552: f64, t28: f64, t1649: f64, t7541: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64, t504: f64, t1972: f64, t52: f64, rho1: f64, t1873: f64, t4028: f64, t1458: f64, t88: f64, t1268: f64, t7467: f64, t6517: f64, t7451: f64, t1778: f64, t191: f64, t192: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t7627 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1263(t1052, t1635, t1920, t1956, t388, t4557, t4660, t6685, t6687, t6771, t7554, t7557, t7562, t7566, t7569, t7594, t7600, t7625);
        let (t7637, t7642, t7643) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1264(t265, t394, t1484, t1915, t202, t7540, t1530, t1877, t193, t2522, t6670, t870, t1070, t1637, t336, t4700, t6822, t7627);
        let (t7648, t7649, t7656, t7663) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1265(t25, t1409, t1965, t40, t7552, t7643, t1484, t28, t1915, t1530, t1649, t1877, t2522, t6670, t7541, dens_threshold, rho0, zeta_threshold);
        let (t7664, t7670) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1266(t28, t265, t504, t7642, t1409, t1972, t52, t7663, t7648, dens_threshold, rho1, zeta_threshold);
        let (t7675, t7676) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1267(t1873, t4028, t1458, t88);
        let (t7681, t7684, t7685) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1268(t1873, t7676, t1268, t7467, t1458, t6517, t7451, t7675, t1778, t191, t192);
    (t7627, t7637, t7643, t7649, t7656, t7664, t7670, t7676, t7681, t7684, t7685)
}
