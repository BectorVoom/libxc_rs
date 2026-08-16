//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta269 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1292;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1293;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1294;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1295;
use chunk4::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1296;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta269<F: Float>(t1052: F, t1635: F, t1920: F, t1956: F, t388: F, t4557: F, t4660: F, t6685: F, t6687: F, t6771: F, t7554: F, t7557: F, t7562: F, t7566: F, t7569: F, t7594: F, t7600: F, t7625: F, t265: F, t394: F, t1484: F, t1915: F, t202: F, t7540: F, t1530: F, t1877: F, t193: F, t2522: F, t6670: F, t870: F, t1070: F, t1637: F, t336: F, t4700: F, t6822: F, t25: F, t1409: F, t1965: F, t40: F, t7552: F, t28: F, t1649: F, t7541: F, dens_threshold: F, rho0: F, zeta_threshold: F, t504: F, t1972: F, t52: F, rho1: F, t1873: F, t4028: F, t1458: F, t88: F, t1268: F, t7467: F, t6517: F, t7451: F, t1778: F, t191: F, t192: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t7627 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1292::<F>(t1052, t1635, t1920, t1956, t388, t4557, t4660, t6685, t6687, t6771, t7554, t7557, t7562, t7566, t7569, t7594, t7600, t7625);
        let (t7634, t7642, t7643) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1293::<F>(t265, t394, t1484, t1915, t202, t7540, t1530, t1877, t193, t2522, t6670, t870, t1070, t1637, t336, t4700, t6822, t7627);
        let (t7648, t7649, t7650, t7656, t7663) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1294::<F>(t25, t1409, t1965, t40, t7552, t7643, t1484, t28, t1915, t1530, t1649, t1877, t2522, t6670, t7541, dens_threshold, rho0, zeta_threshold);
        let (t7664, t7670) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1295::<F>(t28, t265, t504, t7642, t1409, t1972, t52, t7663, t7648, dens_threshold, rho1, zeta_threshold);
        let (t7676, t7681, t7684, t7685) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1296::<F>(t1873, t4028, t1458, t88, t1268, t7467, t6517, t7451, t1778, t191, t192);
    (t7627, t7634, t7643, t7649, t7650, t7656, t7664, t7670, t7676, t7681, t7684, t7685)
}
