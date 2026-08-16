//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta637 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2149;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2150;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2151;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2152;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta637(t23133: f64, t4257: f64, t1496: f64, t81942: f64, t7497: f64, t81933: f64, t25098: f64, t81835: f64, t13228: f64, t2628: f64, t2678: f64, t6605: f64, t23097: f64, t4234: f64, t776: f64, t815: f64, t81877: f64, t81883: f64, t13176: f64, t6620: f64, t849: f64, t81857: f64, t81859: f64, t81874: f64, t87287: f64, t87289: f64, t87292: f64, t87293: f64, t87296: f64, t87298: f64, t25097: f64, t81782: f64, t81783: f64, t1516: f64, t81769: f64, t4261: f64, t25111: f64, t25115: f64, t87229: f64, t23132: f64, t4166: f64, t25068: f64, t2707: f64, t81763: f64, t23083: f64, t25094: f64, t1510: f64, t2379: f64, t25119: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t87301, t87304, t87306, t87308, t87312) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2149(t23133, t4257, t1496, t81942, t7497, t81933, t25098, t81835, t13228, t2628, t2678, t6605);
        let t87324 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2150(t23097, t4234, t776, t815, t81877, t81883, t13176, t6620, t849, t81857, t81859, t81874, t87287, t87289, t87292, t87293, t87296, t87298, t87301, t87304, t87306, t87308, t87312);
        let (t87329, t87331, t87333, t87336, t87339, t87340) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2151(t25097, t81782, t81783, t1516, t81769, t23133, t4261, t25111, t25115, t87229, t23132, t4166);
        let (t87342, t87343, t87345, t87348, t87351) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2152(t849, t87340, t25068, t2707, t1516, t81763, t23083, t25094, t1510, t2379, t25119, t815);
    (t87324, t87329, t87331, t87333, t87336, t87339, t87342, t87343, t87345, t87348, t87351)
}
