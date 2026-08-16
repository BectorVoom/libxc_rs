//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta732 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2401;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2402;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2403;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta732<F: Float>(t17934: F, t4493: F, t21697: F, t3216: F, t17299: F, t4483: F, t14473: F, t5812: F, t41684: F, t47706: F, t47707: F, t47731: F, t59657: F, t68442: F, t68444: F, t68446: F, t68448: F, t68479: F, t68483: F, t68486: F, t68489: F, t68492: F, t68494: F, t68498: F, t68571: F, t68577: F, t68580: F, t68583: F, t41741: F, t47787: F, t59663: F, t59665: F, t59680: F, t59688: F, t59694: F, t59700: F, t59702: F, t59704: F, t59759: F, t59761: F, t68586: F, t68589: F, t68592: F, t68596: F, t68599: F, t68602: F, t68605: F, t68608: F, t324: F, t300: F, t1557: F, t59979: F, t17195: F, t4396: F, t1068: F, t25845: F, t4700: F, t60874: F, t68441: F, t68706: F, t68708: F) -> (F, F, F, F, F, F, F, F) {
        let (t68710, t68711, t68715, t68717, t68736) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2401::<F>(t17934, t4493, t21697, t3216, t17299, t4483, t14473, t5812, t41684, t47706, t47707, t47731, t59657, t68442, t68444, t68446, t68448, t68479, t68483, t68486, t68489, t68492, t68494, t68498, t68571, t68577, t68580, t68583);
        let t68756 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2402::<F>(t41741, t47787, t59663, t59665, t59680, t59688, t59694, t59700, t59702, t59704, t59759, t59761, t68586, t68589, t68592, t68596, t68599, t68602, t68605, t68608);
        let (t68758, t68760, t68762, t68764, t68765) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2403::<F>(t324, t68736, t68756, t300, t1557, t59979, t17195, t4396, t1068, t25845, t4700, t60874, t68441, t68706, t68708, t68710, t68711, t68715, t68717);
    (t68710, t68715, t68717, t68758, t68760, t68762, t68764, t68765)
}
