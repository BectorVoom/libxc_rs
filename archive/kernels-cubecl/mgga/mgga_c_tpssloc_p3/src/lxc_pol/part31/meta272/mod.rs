//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta272 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1126;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1127;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1128;
use chunk3::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1129;
use chunk4::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1130;
use chunk5::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1131;
use chunk6::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1132;
use chunk7::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1133;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta272<F: Float>(t7841: F, t858: F, t1528: F, t2054: F, t259: F, t4147: F, t4268: F, t7067: F, t7069: F, t7087: F, t7481: F, t7486: F, t7490: F, t7815: F, t7824: F, t7830: F, t855: F, t870: F, t25: F, t265: F, t394: F, t1484: F, t2057: F, t202: F, t1530: F, t1877: F, t193: F, t2522: F, t7114: F, t1408: F, t1409: F, t2064: F, t40: F, t7545: F, t7809: F, dens_threshold: F, rho0: F, zeta_threshold: F, t28: F, t504: F, t7649: F, t1649: F, t2071: F, t52: F, t7656: F, rho1: F, t1268: F, t1458: F, t2039: F, t4028: F, t7042: F, t7676: F, t7787: F, t7801: F, t7170: F, t7687: F, t1807: F, t2085: F, t7181: F, t7183: F, t7185: F, t7189: F, t7706: F, t7710: F, t7713: F, t7716: F, t7718: F, t7720: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let t7842 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1126::<F>(t7841, t858);
        let t7844 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1127::<F>(t1528, t2054, t259, t4147, t4268, t7067, t7069, t7087, t7481, t7486, t7490, t7815, t7824, t7830, t7842, t855);
        let t7845 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1128::<F>(t7844, t870);
        let (t7859, t7864, t7865, t7870) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1129::<F>(t25, t265, t394, t1484, t2057, t202, t7844, t1530, t1877, t193, t2522, t7114, t870, t1408, t1409, t2064, t40, t7545, t7809, t7845, dens_threshold, rho0, zeta_threshold);
        let (t7884, t7889) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1130::<F>(t28, t265, t504, t2057, t7649, t7864, t1409, t1649, t1877, t2071, t2522, t52, t7114, t7656, t7845, dens_threshold, rho1, zeta_threshold);
        let t7890 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1131::<F>(t7870, t7889);
        let (t7900, t7904, t7910) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1132::<F>(t1268, t1458, t2039, t4028, t7042, t7676, t7787, t7801, t7170, t7687, t1807, t2085);
        let t7918 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1133::<F>(t7181, t7183, t7185, t7189, t7706, t7710, t7713, t7716, t7718, t7720);
    (t7842, t7844, t7845, t7859, t7865, t7884, t7890, t7900, t7904, t7910, t7918)
}
