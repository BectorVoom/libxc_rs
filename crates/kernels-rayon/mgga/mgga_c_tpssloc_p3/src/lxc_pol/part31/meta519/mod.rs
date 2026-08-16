//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta519 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1724;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1725;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1726;
use chunk3::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1727;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta519(t2057: f64, t5527: f64, t1484: f64, t1530: f64, t1877: f64, t193: f64, t202: f64, t24344: f64, t2522: f64, t26744: f64, t28248: f64, t29105: f64, t4314: f64, t5544: f64, t5660: f64, t5664: f64, t7114: f64, t7845: f64, t870: f64, t25: f64, t265: f64, t394: f64, t1409: f64, t2064: f64, t29124: f64, t40: f64, t5398: f64, t7865: f64, t28764: f64, t1649: f64, t24191: f64, t28: f64, t28771: f64, t28774: f64, t28778: f64, t28789: f64, t28792: f64, t28795: f64, t29106: f64, t5966: f64, t7649: f64, t7656: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64, t504: f64, t2071: f64, t52: f64, t7884: f64, t5161: f64, t7940: f64, t1458: f64, t7890: f64, rho1: f64, t113: f64, t1442: f64, t1459: f64, t1774: f64, t1849: f64, t1983: f64, t2075: f64, t2096: f64, t27188: f64, t28821: f64, t28943: f64, t28952: f64, t28959: f64, t28969: f64, t4028: f64, t510: f64, t5450: f64, t5457: f64, t5460: f64, t5494: f64, t652: f64, t7042: f64, t7458: f64, t7685: f64, t7787: f64, t7802: f64, t7806: f64, t7900: f64, t7941: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t29125, t29148) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1724(t2057, t5527, t1484, t1530, t1877, t193, t202, t24344, t2522, t26744, t28248, t29105, t4314, t5544, t5660, t5664, t7114, t7845, t870);
        let (t29149, t29156, t29157, t29188) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1725(t25, t265, t394, t29148, t1409, t2064, t29124, t40, t5398, t7865, t2057, t28764, t1649, t1877, t24191, t24344, t2522, t26744, t28, t28771, t28774, t28778, t28789, t28792, t28795, t29106, t4314, t5966, t7114, t7649, t7656, t7845, dens_threshold, rho0, zeta_threshold);
        let (t29189, t29197, t29201, t29205) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1726(t28, t265, t504, t29148, t1409, t2071, t29188, t52, t5398, t7884, t29156, t5161, t7940, t1458, t7890, dens_threshold, rho1, zeta_threshold);
        let t29210 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1727(t113, t1442, t1459, t1774, t1849, t1983, t2075, t2096, t27188, t28821, t28943, t28952, t28959, t28969, t29197, t29201, t29205, t4028, t510, t5450, t5457, t5460, t5494, t652, t7042, t7458, t7685, t7787, t7802, t7806, t7890, t7900, t7941);
    (t29125, t29149, t29157, t29189, t29197, t29201, t29205, t29210)
}
