//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta647 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2375;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2376;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2377;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2378;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta647(t10662: f64, t14395: f64, t42028: f64, t10829: f64, t14258: f64, t959: f64, t10605: f64, t4483: f64, t10523: f64, t2933: f64, t4471: f64, t47793: f64, t47795: f64, t47798: f64, t47802: f64, t48679: f64, t48681: f64, t48725: f64, t48727: f64, t48730: f64, t48732: f64, t48734: f64, t48736: f64, t48738: f64, t48741: f64, t48744: f64, t48747: f64, t14473: f64, t2944: f64, t10661: f64, t1556: f64, t10731: f64, t14363: f64, t300: f64, t961: f64, t2948: f64, t14419: f64, t923: f64, t10771: f64, t1568: f64, t10756: f64, t1580: f64, t2930: f64, t10717: f64, t10720: f64, t10744: f64, t14271: f64, t42671: f64, t933: f64, t950: f64, t2885: f64, t4408: f64, t47705: f64, t47707: f64, t47730: f64, t47681: f64, t47686: f64, t47691: f64, t47695: f64, t47699: f64, t47703: f64, t47709: f64, t47711: f64, t47713: f64, t47715: f64, t47717: f64, t47722: f64, t47724: f64, t47728: f64, t47732: f64, t47736: f64, t47738: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t48750, t48753, t48755, t48759, t48760) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2375(t10662, t14395, t42028, t10829, t14258, t959, t10605, t4483, t10523, t2933, t4471, t47793, t47795, t47798, t47802, t48679, t48681, t48725, t48727, t48730, t48732, t48734, t48736, t48738, t48741, t48744, t48747);
        let (t48762, t48765, t48768, t48770, t48771, t48776) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2376(t14473, t2944, t10661, t1556, t10731, t14363, t300, t961, t2948, t14419, t923, t10771, t1568);
        let t48786 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2377(t10756, t1580, t2930, t10717, t10720, t10744, t14271, t42671, t47798, t47802, t48725, t48730, t48732, t48734, t48736, t48738, t48741, t48744, t48771, t48776, t933, t950);
        let (t48789, t48813) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2378(t2885, t4408, t47705, t47707, t47730, t47681, t47686, t47691, t47695, t47699, t47703, t47709, t47711, t47713, t47715, t47717, t47722, t47724, t47728, t47732, t47736, t47738);
    (t48750, t48753, t48755, t48759, t48760, t48762, t48765, t48768, t48770, t48786, t48789, t48813)
}
