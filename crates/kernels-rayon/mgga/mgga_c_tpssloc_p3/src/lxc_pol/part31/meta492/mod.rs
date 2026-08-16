//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta492 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1678;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1679;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1680;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta492(t26429: f64, t1338: f64, t7918: f64, t1352: f64, t5287: f64, t7208: f64, t27051: f64, t553: f64, t1332: f64, t1336: f64, t1814: f64, t2089: f64, t22728: f64, t22731: f64, t22746: f64, t22753: f64, t22896: f64, t24108: f64, t24110: f64, t26434: f64, t26437: f64, t26449: f64, t26463: f64, t26468: f64, t5230: f64, t544: f64, t7211: f64, t7934: f64, t27095: f64, t1378: f64, t1375: f64, t1386: f64, t16022: f64, t16439: f64, t1843: f64, t2092: f64, t22676: f64, t24095: f64, t26475: f64, t27067: f64, t27068: f64, t27070: f64, t3758: f64, t3882: f64, t5215: f64, t5321: f64, t568: f64, t7199: f64, t7214: f64, t7937: f64, t1842: f64, t7213: f64, t3887: f64, t1807: f64, t7191: f64, t16460: f64, t22908: f64, t22910: f64, t22922: f64, t22928: f64, t22941: f64, t24082: f64, t24156: f64, t24157: f64, t5354: f64, t7194: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t27097, t27098, t27103, t27105, t27113) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1678(t26429, t1338, t7918, t1352, t5287, t7208, t27051, t553, t1332, t1336, t1814, t2089, t22728, t22731, t22746, t22753, t22896, t24108, t24110, t26434, t26437, t26449, t26463, t26468, t5230, t544, t7211, t7934);
        let (t27114, t27115, t27127) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1679(t27095, t27113, t1378, t1375, t1386, t16022, t16439, t1843, t2092, t22676, t24095, t26475, t27067, t27068, t27070, t3758, t3882, t5215, t5321, t568, t7199, t7214, t7937);
        let (t27132, t27137, t27141) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1680(t1842, t7213, t3887, t1807, t7191, t1375, t16460, t1843, t2092, t22908, t22910, t22922, t22928, t22941, t24082, t24156, t24157, t5215, t5321, t5354, t568, t7194, t7199, t7214);
    (t27097, t27098, t27103, t27105, t27114, t27115, t27127, t27132, t27137, t27141)
}
