//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta522 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1734;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1735;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1736;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta522(t6420: f64, t7208: f64, t6415: f64, t1825: f64, t27097: f64, t1336: f64, t24108: f64, t24110: f64, t26427: f64, t26429: f64, t26437: f64, t28161: f64, t28165: f64, t28169: f64, t28183: f64, t5234: f64, t7932: f64, t29342: f64, t1378: f64, t2091: f64, t3887: f64, t6460: f64, t1375: f64, t20029: f64, t20044: f64, t20060: f64, t2092: f64, t24156: f64, t24157: f64, t26361: f64, t26475: f64, t28207: f64, t28211: f64, t28214: f64, t28234: f64, t5215: f64, t5321: f64, t6440: f64, t6461: f64, t7194: f64, t7925: f64, t7937: f64, t29314: f64, t533: f64, t1390: f64, t26905: f64, t7687: f64, t19451: f64, t1983: f64, t2036: f64, t2040: f64, t2079: f64, t22574: f64, t28002: f64, t28030: f64, t29211: f64, t29214: f64, t29219: f64, t29222: f64, t29241: f64, t29243: f64, t29247: f64, t29252: f64, t4028: f64, t574: f64, t6287: f64, t6468: f64, t652: f64, t7458: f64, t7685: f64, t7796: f64, t7802: f64, t7904: f64, t7943: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t29343, t29345, t29349, t29359) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1734(t6420, t7208, t6415, t1825, t27097, t1336, t24108, t24110, t26427, t26429, t26437, t28161, t28165, t28169, t28183, t5234, t7932);
        let (t29360, t29361, t29372, t29375) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1735(t29342, t29359, t1378, t2091, t3887, t6460, t1375, t20029, t20044, t20060, t2092, t24156, t24157, t26361, t26475, t28207, t28211, t28214, t28234, t5215, t5321, t6440, t6461, t7194, t7925, t7937);
        let (t29376, t29377, t29378, t29380, t29394) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1736(t29314, t29375, t533, t1390, t26905, t7687, t19451, t1983, t2036, t2040, t2079, t22574, t28002, t28030, t29211, t29214, t29219, t29222, t29241, t29243, t29247, t29252, t4028, t574, t6287, t6468, t652, t7458, t7685, t7796, t7802, t7904, t7943);
    (t29343, t29345, t29349, t29360, t29361, t29372, t29376, t29377, t29378, t29380, t29394)
}
