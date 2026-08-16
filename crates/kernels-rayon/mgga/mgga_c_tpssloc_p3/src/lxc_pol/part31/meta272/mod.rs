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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1126;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1127;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1128;
use chunk3::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1129;
use chunk4::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1130;
use chunk5::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1131;
use chunk6::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1132;
use chunk7::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1133;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta272(t7841: f64, t858: f64, t1528: f64, t2054: f64, t259: f64, t4147: f64, t4268: f64, t7067: f64, t7069: f64, t7087: f64, t7481: f64, t7486: f64, t7490: f64, t7815: f64, t7824: f64, t7830: f64, t855: f64, t870: f64, t25: f64, t265: f64, t394: f64, t1484: f64, t2057: f64, t202: f64, t1530: f64, t1877: f64, t193: f64, t2522: f64, t7114: f64, t1408: f64, t1409: f64, t2064: f64, t40: f64, t7545: f64, t7809: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64, t28: f64, t504: f64, t7649: f64, t1649: f64, t2071: f64, t52: f64, t7656: f64, rho1: f64, t1268: f64, t1458: f64, t2039: f64, t4028: f64, t7042: f64, t7676: f64, t7787: f64, t7801: f64, t7170: f64, t7687: f64, t1807: f64, t2085: f64, t7181: f64, t7183: f64, t7185: f64, t7189: f64, t7706: f64, t7710: f64, t7713: f64, t7716: f64, t7718: f64, t7720: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t7842 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1126(t7841, t858);
        let t7844 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1127(t1528, t2054, t259, t4147, t4268, t7067, t7069, t7087, t7481, t7486, t7490, t7815, t7824, t7830, t7842, t855);
        let t7845 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1128(t7844, t870);
        let (t7859, t7864, t7865, t7870) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1129(t25, t265, t394, t1484, t2057, t202, t7844, t1530, t1877, t193, t2522, t7114, t870, t1408, t1409, t2064, t40, t7545, t7809, t7845, dens_threshold, rho0, zeta_threshold);
        let (t7884, t7889) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1130(t28, t265, t504, t2057, t7649, t7864, t1409, t1649, t1877, t2071, t2522, t52, t7114, t7656, t7845, dens_threshold, rho1, zeta_threshold);
        let t7890 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1131(t7870, t7889);
        let (t7900, t7904, t7910) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1132(t1268, t1458, t2039, t4028, t7042, t7676, t7787, t7801, t7170, t7687, t1807, t2085);
        let t7918 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1133(t7181, t7183, t7185, t7189, t7706, t7710, t7713, t7716, t7718, t7720);
    (t7842, t7844, t7845, t7859, t7865, t7884, t7890, t7900, t7904, t7910, t7918)
}
