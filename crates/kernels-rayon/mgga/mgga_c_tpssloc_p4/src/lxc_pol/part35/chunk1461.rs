//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1461/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1461(t28: f64, t106717: f64, t109953: f64, t1409: f64, t20217: f64, t2161: f64, t29840: f64, t52: f64, t5398: f64, t8097: f64, t106747: f64, t106753: f64, t106756: f64, t106889: f64, t106891: f64, t106895: f64, t106899: f64, t106901: f64, t106905: f64, t106919: f64, t106964: f64, t109029: f64, t109055: f64, t113: f64, t1458: f64, t1774: f64, t20293: f64, t20702: f64, t2165: f64, t29486: f64, t29501: f64, t29848: f64, t29855: f64, t4028: f64, t510: f64, t652: f64, t7266: f64, t7458: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64) -> f64 {
    let t29 = t28 <= zeta_threshold;
    let t401 = rho1 <= dens_threshold || t29;
    let t109963 = piecewise3(t401, t106717, t109953 * t52 / 2.0_f64 - 3.0_f64 / 2.0_f64 * t29840 * t1409 - 3.0_f64 / 2.0_f64 * t8097 * t5398 - t2161 * t20217 / 2.0_f64);
    let t109966 = -t106747 - t109029 * t510 - 6.0_f64 * t4028 * t29855 + t106753 + t106756 - 6.0_f64 * t652 * t29848 * t1458 - t20293 * t2165 - 6.0_f64 * t7266 * t20702 - t106889 - t106891 - t106895 - t106899 - t106901 - t106905 - 6.0_f64 * t7458 * t29855 - 12.0_f64 * t4028 * t29501 - 3.0_f64 * t29486 * t1774 - t106919 - t106964 - t113 * (t109055 + t109963);
    t109966
}
