//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta273 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1134;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1135;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1136;
use chunk3::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1137;
use chunk4::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1138;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta273(t539: f64, t7918: f64, t1842: f64, t2091: f64, t3887: f64, t1825: f64, t7208: f64, t553: f64, t1336: f64, t1814: f64, t2089: f64, t544: f64, t7202: f64, t7204: f64, t7734: f64, t7738: f64, t7742: f64, t1378: f64, t1375: f64, t1843: f64, t2092: f64, t5215: f64, t5321: f64, t568: f64, t7174: f64, t7176: f64, t7194: f64, t7693: f64, t7698: f64, t7702: f64, t7910: f64, t533: f64, t1390: f64, t2095: f64, t5161: f64, t113: f64, t1442: f64, t1459: f64, t1774: f64, t1849: f64, t1983: f64, t2036: f64, t2040: f64, t2075: f64, t2079: f64, t2096: f64, t4028: f64, t510: f64, t574: f64, t652: f64, t7042: f64, t7458: f64, t7685: f64, t7787: f64, t7796: f64, t7802: f64, t7806: f64, t7890: f64, t7900: f64, t7904: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t7919, t7925) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1134(t539, t7918, t1842, t2091, t3887);
        let (t7932, t7934, t7936) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1135(t1825, t7208, t553, t7918, t1336, t1814, t2089, t544, t7202, t7204, t7734, t7738, t7742);
        let t7937 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1136(t1378, t7936);
        let t7939 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1137(t1375, t1843, t2092, t5215, t5321, t568, t7174, t7176, t7194, t7693, t7698, t7702, t7910, t7919, t7925, t7937);
        let (t7940, t7941, t7943, t7945) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1138(t533, t7939, t1390, t2095, t5161, t113, t1442, t1459, t1774, t1849, t1983, t2036, t2040, t2075, t2079, t2096, t4028, t510, t574, t652, t7042, t7458, t7685, t7787, t7796, t7802, t7806, t7890, t7900, t7904);
    (t7919, t7925, t7932, t7934, t7936, t7937, t7939, t7940, t7941, t7943, t7945)
}
