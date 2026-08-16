//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta732 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2401;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2402;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2403;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta732(t17934: f64, t4493: f64, t21697: f64, t3216: f64, t17299: f64, t4483: f64, t14473: f64, t5812: f64, t41684: f64, t47706: f64, t47707: f64, t47731: f64, t59657: f64, t68442: f64, t68444: f64, t68446: f64, t68448: f64, t68479: f64, t68483: f64, t68486: f64, t68489: f64, t68492: f64, t68494: f64, t68498: f64, t68571: f64, t68577: f64, t68580: f64, t68583: f64, t41741: f64, t47787: f64, t59663: f64, t59665: f64, t59680: f64, t59688: f64, t59694: f64, t59700: f64, t59702: f64, t59704: f64, t59759: f64, t59761: f64, t68586: f64, t68589: f64, t68592: f64, t68596: f64, t68599: f64, t68602: f64, t68605: f64, t68608: f64, t324: f64, t300: f64, t1557: f64, t59979: f64, t17195: f64, t4396: f64, t1068: f64, t25845: f64, t4700: f64, t60874: f64, t68441: f64, t68706: f64, t68708: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t68710, t68711, t68715, t68717, t68736) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2401(t17934, t4493, t21697, t3216, t17299, t4483, t14473, t5812, t41684, t47706, t47707, t47731, t59657, t68442, t68444, t68446, t68448, t68479, t68483, t68486, t68489, t68492, t68494, t68498, t68571, t68577, t68580, t68583);
        let t68756 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2402(t41741, t47787, t59663, t59665, t59680, t59688, t59694, t59700, t59702, t59704, t59759, t59761, t68586, t68589, t68592, t68596, t68599, t68602, t68605, t68608);
        let (t68758, t68760, t68762, t68764, t68765) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2403(t324, t68736, t68756, t300, t1557, t59979, t17195, t4396, t1068, t25845, t4700, t60874, t68441, t68706, t68708, t68710, t68711, t68715, t68717);
    (t68710, t68715, t68717, t68758, t68760, t68762, t68764, t68765)
}
