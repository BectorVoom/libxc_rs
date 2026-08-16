//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta270 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1153;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1154;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1155;
use chunk3::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1156;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta270<F: Float>(t1825: F, t7208: F, t553: F, t7918: F, t1336: F, t1814: F, t2089: F, t544: F, t7202: F, t7204: F, t7734: F, t7738: F, t7742: F, t1378: F, t1375: F, t1843: F, t2092: F, t5215: F, t5321: F, t568: F, t7174: F, t7176: F, t7194: F, t7693: F, t7698: F, t7702: F, t7910: F, t7919: F, t7925: F, t533: F, t1390: F, t2095: F, t5161: F, t113: F, t1442: F, t1459: F, t1774: F, t1849: F, t1983: F, t2036: F, t2040: F, t2075: F, t2079: F, t2096: F, t4028: F, t510: F, t574: F, t652: F, t7042: F, t7458: F, t7685: F, t7787: F, t7796: F, t7802: F, t7806: F, t7890: F, t7900: F, t7904: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t7932, t7934, t7936) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1153::<F>(t1825, t7208, t553, t7918, t1336, t1814, t2089, t544, t7202, t7204, t7734, t7738, t7742);
        let t7937 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1154::<F>(t1378, t7936);
        let t7939 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1155::<F>(t1375, t1843, t2092, t5215, t5321, t568, t7174, t7176, t7194, t7693, t7698, t7702, t7910, t7919, t7925, t7937);
        let (t7940, t7941, t7943, t7945) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1156::<F>(t533, t7939, t1390, t2095, t5161, t113, t1442, t1459, t1774, t1849, t1983, t2036, t2040, t2075, t2079, t2096, t4028, t510, t574, t652, t7042, t7458, t7685, t7787, t7796, t7802, t7806, t7890, t7900, t7904);
    (t7932, t7934, t7936, t7937, t7939, t7940, t7941, t7943, t7945)
}
