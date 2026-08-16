//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta576 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1951;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1952;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta576(t1484: f64, t1649: f64, t28: f64, t5544: f64, t5664: f64, t1530: f64, t5660: f64, t1877: f64, t1915: f64, t22959: f64, t23295: f64, t2522: f64, t25358: f64, t28448: f64, t28765: f64, t28771: f64, t4314: f64, t5966: f64, t6670: f64, t7541: f64, t7649: f64, t7656: f64, t265: f64, t504: f64, t28755: f64, t1409: f64, t1972: f64, t52: f64, t5398: f64, t7664: f64, t28763: f64, t5161: f64, t7753: f64, t1983: f64, t113: f64, t1459: f64, t1980: f64, t24999: f64, t27993: f64, t27996: f64, t28020: f64, t28027: f64, t28029: f64, t28032: f64, t28034: f64, t28036: f64, t28038: f64, t28040: f64, t28042: f64, t28047: f64, t28240: f64, t510: f64, t5460: f64, t5494: f64, t574: f64, t6468: f64, t6517: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t28774, t28778, t28789, t28792, t28795, t28802) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1951(t1484, t1649, t28, t5544, t5664, t1530, t5660, t1877, t1915, t22959, t23295, t2522, t25358, t28448, t28765, t28771, t4314, t5966, t6670, t7541, t7649, t7656);
        let (t28803, t28811, t28813, t28816) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1952(t28, t265, t504, t28755, t1409, t1972, t28802, t52, t5398, t7664, t28763, t5161, t7753, t1983, t113, t1459, t1980, t24999, t27993, t27996, t28020, t28027, t28029, t28032, t28034, t28036, t28038, t28040, t28042, t28047, t28240, t510, t5460, t5494, t574, t6468, t6517, dens_threshold, rho1, zeta_threshold);
    (t28774, t28778, t28789, t28792, t28795, t28803, t28811, t28813, t28816)
}
