//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta191 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1128;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1129;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1130;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1131;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1132;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta191(t2701: f64, t5527: f64, t820: f64, t5544: f64, t847: f64, t1512: f64, t1516: f64, t249: f64, t2571: f64, t2602: f64, t2630: f64, t2643: f64, t2695: f64, t4152: f64, t4167: f64, t4170: f64, t4172: f64, t4187: f64, t4253: f64, t5568: f64, t5572: f64, t5576: f64, t5587: f64, t5593: f64, t5614: f64, t5619: f64, t787: f64, t817: f64, t843: f64, t218: f64, t1527: f64, t2718: f64, t2728: f64, t5585: f64, t1510: f64, t4295: f64, t5612: f64, t860: f64, t5617: f64, t235: f64, t1499: f64, t1523: f64, t1525: f64, t226: f64, t255: f64, t4166: f64, t5575: f64, t812: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t5624 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1128(t2701, t5527, t820);
        let t5628 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1129(t5544, t820, t847);
        let t5631 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1130(t1512, t1516, t249, t2571, t2602, t2630, t2643, t2695, t4152, t4167, t4170, t4172, t4187, t4253, t5568, t5572, t5576, t5587, t5593, t5614, t5619, t5624, t5628, t787, t817, t843);
        let (t5632, t5636, t5637) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1131(t218, t5631, t1527, t2718);
        let (t5645, t5648, t5651, t5653, t5655, t5657) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1132(t2728, t5585, t1510, t4295, t5612, t860, t5617, t235, t5631, t1499, t1523, t1525, t226, t255, t4166, t5575, t812);
    (t5624, t5628, t5631, t5632, t5636, t5637, t5645, t5648, t5651, t5653, t5655, t5657)
}
