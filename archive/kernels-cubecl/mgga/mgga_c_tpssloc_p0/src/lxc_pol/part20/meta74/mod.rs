//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta74 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk535;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk536;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk537;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk538;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk539;
use chunk5::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk540;
use chunk6::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk541;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta74<F: Float>(t1484: F, t820: F, t847: F, t1496: F, t1500: F, t1512: F, t249: F, t787: F, t803: F, t817: F, t840: F, t843: F, t218: F, t1510: F, t860: F, t235: F, t1499: F, t226: F, t255: F, t812: F, t858: F, t1493: F, t259: F, t855: F, t1464: F, t1473: F, t1476: F, t193: F, t202: F, t680: F, t705: F, t752: F, t760: F, t765: F, t766: F, t870: F, t1409: F, t883: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let t1516 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk535::<F>(t1484, t820, t847);
        let t1519 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk536::<F>(t1496, t1500, t1512, t1516, t249, t787, t803, t817, t840, t843);
        let (t1520, t1523, t1525, t1527) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk537::<F>(t1519, t218, t1510, t860, t235, t1499, t226, t255, t812);
        let t1528 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk538::<F>(t1527, t858);
        let t1530 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk539::<F>(t1493, t1520, t1528, t259, t855);
        let t1534 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk540::<F>(t1464, t1473, t1476, t1484, t1530, t193, t202, t680, t705, t752, t760, t765, t766, t870);
        let t1539 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk541::<F>(t1409, t883);
    (t1516, t1519, t1520, t1523, t1525, t1527, t1528, t1530, t1534, t1539)
}
