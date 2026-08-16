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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1014;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1015;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1016;
use chunk3::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1017;
use chunk4::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1018;
use chunk5::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1019;
use chunk6::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1020;
use chunk7::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1021;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta242<F: Float>(t1378: F, t6460: F, t1375: F, t1843: F, t5215: F, t5321: F, t568: F, t6362: F, t6364: F, t6435: F, t6440: F, t1297: F, t1390: F, t193: F, t2486: F, t3701: F, t3819: F, t3821: F, t3823: F, t3825: F, t3832: F, t3836: F, t3924: F, t533: F, t6324: F, t6329: F, t6330: F, t6347: F, t6399: F, t6400: F, t6323: F, t113: F, t1442: F, t1459: F, t1774: F, t1778: F, t1849: F, t4028: F, t510: F, t513: F, t5450: F, t5457: F, t5460: F, t5494: F, t574: F, t6287: F, t6295: F, t652: F, t3: F, t1401: F, t1458: F, t3941: F, t5371: F, t5456: F, t5493: F, t577: F, t2235: F, t33: F, t645: F, t79: F, t72: F, t605: F, t608: F, t641: F, t71: F, t107: F, t625: F, t63: F, t656: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t6461 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1014::<F>(t1378, t6460);
        let t6463 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1015::<F>(t1375, t1843, t5215, t5321, t568, t6362, t6364, t6435, t6440, t6461);
        let t6467 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1016::<F>(t1297, t1390, t193, t2486, t3701, t3819, t3821, t3823, t3825, t3832, t3836, t3924, t533, t6324, t6329, t6330, t6347, t6399, t6400, t6463);
        let (t6468, t6470) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1017::<F>(t6323, t6467, t113, t1442, t1459, t1774, t1778, t1849, t4028, t510, t513, t5450, t5457, t5460, t5494, t574, t6287, t6295, t652);
        let (t6471, t6483, t6486) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1018::<F>(t3, t6470, t1401, t1458, t3941, t5371, t5456, t5493, t577, t2235, t33);
        let t6492 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1019::<F>(t645, t79, t72);
        let t6495 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1020::<F>(t605, t608);
        let (t6509, t6528, t6530) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1021::<F>(t641, t71, t107, t625, t63, t656);
    (t6461, t6463, t6468, t6470, t6471, t6483, t6486, t6492, t6495, t6509, t6528, t6530)
}
