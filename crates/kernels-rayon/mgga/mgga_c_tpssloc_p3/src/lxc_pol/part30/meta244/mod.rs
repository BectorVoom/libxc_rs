//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta244 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1098;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1099;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1100;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1101;
use chunk4::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1102;
use chunk5::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1103;
use chunk6::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1104;
use chunk7::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1105;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta244(t1378: f64, t6460: f64, t1375: f64, t1843: f64, t5215: f64, t5321: f64, t568: f64, t6362: f64, t6364: f64, t6435: f64, t6440: f64, t1297: f64, t1390: f64, t193: f64, t2486: f64, t3701: f64, t3819: f64, t3821: f64, t3823: f64, t3825: f64, t3832: f64, t3836: f64, t3924: f64, t533: f64, t6324: f64, t6329: f64, t6330: f64, t6347: f64, t6399: f64, t6400: f64, t6323: f64, t113: f64, t1442: f64, t1459: f64, t1774: f64, t1778: f64, t1849: f64, t4028: f64, t510: f64, t513: f64, t5450: f64, t5457: f64, t5460: f64, t5494: f64, t574: f64, t6287: f64, t6295: f64, t652: f64, t3: f64, t1401: f64, t1458: f64, t3941: f64, t5371: f64, t5456: f64, t5493: f64, t577: f64, t2235: f64, t33: f64, t1862: f64, t2240: f64, t645: f64, t79: f64, t72: f64, t605: f64, t608: f64, t38: f64, t43: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t6461 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1098(t1378, t6460);
        let t6463 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1099(t1375, t1843, t5215, t5321, t568, t6362, t6364, t6435, t6440, t6461);
        let t6467 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1100(t1297, t1390, t193, t2486, t3701, t3819, t3821, t3823, t3825, t3832, t3836, t3924, t533, t6324, t6329, t6330, t6347, t6399, t6400, t6463);
        let (t6468, t6470) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1101(t6323, t6467, t113, t1442, t1459, t1774, t1778, t1849, t4028, t510, t513, t5450, t5457, t5460, t5494, t574, t6287, t6295, t652);
        let (t6471, t6483, t6486, t6489) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1102(t3, t6470, t1401, t1458, t3941, t5371, t5456, t5493, t577, t2235, t33, t1862);
        let t6490 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1103(t2240, t6489);
        let t6492 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1104(t645, t79, t72);
        let (t6495, t6500) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1105(t605, t608, t38, t43);
    (t6461, t6463, t6468, t6470, t6471, t6483, t6486, t6489, t6490, t6492, t6495, t6500)
}
