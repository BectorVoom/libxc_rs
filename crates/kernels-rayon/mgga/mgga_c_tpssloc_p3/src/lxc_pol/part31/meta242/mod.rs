//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta242 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1014;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1015;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1016;
use chunk3::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1017;
use chunk4::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1018;
use chunk5::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1019;
use chunk6::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1020;
use chunk7::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1021;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta242(t1378: f64, t6460: f64, t1375: f64, t1843: f64, t5215: f64, t5321: f64, t568: f64, t6362: f64, t6364: f64, t6435: f64, t6440: f64, t1297: f64, t1390: f64, t193: f64, t2486: f64, t3701: f64, t3819: f64, t3821: f64, t3823: f64, t3825: f64, t3832: f64, t3836: f64, t3924: f64, t533: f64, t6324: f64, t6329: f64, t6330: f64, t6347: f64, t6399: f64, t6400: f64, t6323: f64, t113: f64, t1442: f64, t1459: f64, t1774: f64, t1778: f64, t1849: f64, t4028: f64, t510: f64, t513: f64, t5450: f64, t5457: f64, t5460: f64, t5494: f64, t574: f64, t6287: f64, t6295: f64, t652: f64, t3: f64, t1401: f64, t1458: f64, t3941: f64, t5371: f64, t5456: f64, t5493: f64, t577: f64, t2235: f64, t33: f64, t645: f64, t79: f64, t72: f64, t605: f64, t608: f64, t641: f64, t71: f64, t107: f64, t625: f64, t63: f64, t656: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t6461 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1014(t1378, t6460);
        let t6463 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1015(t1375, t1843, t5215, t5321, t568, t6362, t6364, t6435, t6440, t6461);
        let t6467 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1016(t1297, t1390, t193, t2486, t3701, t3819, t3821, t3823, t3825, t3832, t3836, t3924, t533, t6324, t6329, t6330, t6347, t6399, t6400, t6463);
        let (t6468, t6470) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1017(t6323, t6467, t113, t1442, t1459, t1774, t1778, t1849, t4028, t510, t513, t5450, t5457, t5460, t5494, t574, t6287, t6295, t652);
        let (t6471, t6483, t6486) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1018(t3, t6470, t1401, t1458, t3941, t5371, t5456, t5493, t577, t2235, t33);
        let t6492 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1019(t645, t79, t72);
        let t6495 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1020(t605, t608);
        let (t6509, t6528, t6530) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1021(t641, t71, t107, t625, t63, t656);
    (t6461, t6463, t6468, t6470, t6471, t6483, t6486, t6492, t6495, t6509, t6528, t6530)
}
