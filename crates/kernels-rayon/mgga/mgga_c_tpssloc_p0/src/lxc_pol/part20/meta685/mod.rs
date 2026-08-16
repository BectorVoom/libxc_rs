//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta685 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2595;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2596;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2597;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2598;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta685(t1227: f64, t49850: f64, t4988: f64, t15568: f64, t3604: f64, t11697: f64, t15473: f64, t3577: f64, t11698: f64, t15740: f64, t10401: f64, t15567: f64, t3610: f64, t11154: f64, t11668: f64, t11680: f64, t11688: f64, t11825: f64, t11863: f64, t15453: f64, t15569: f64, t1735: f64, t3580: f64, t44996: f64, t4582: f64, t48554: f64, t4954: f64, t4989: f64, t5024: f64, t11692: f64, t15563: f64, t15743: f64, t3490: f64, t15239: f64, t486: f64, t15498: f64, t3523: f64, t11734: f64, t1174: f64, t11774: f64, t1216: f64, t15637: f64, t3440: f64, t3515: f64, t44932: f64, t4984: f64, t5005: f64, t50857: f64, t50861: f64, t15495: f64, t3572: f64, t1653: f64, t248: f64, t45293: f64, t15591: f64, t15643: f64, t1089: f64, t3507: f64, t607: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t52610, t52615, t52619, t52621, t52627) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2595(t1227, t49850, t4988, t15568, t3604, t11697, t15473, t3577, t11698, t15740, t10401, t15567);
        let t52639 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2596(t3610, t52627, t11154, t11668, t11680, t11688, t11825, t11863, t1227, t15453, t15569, t1735, t3577, t3580, t44996, t4582, t48554, t4954, t4989, t5024, t52610, t52615, t52619, t52621);
        let (t52659, t52668) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2597(t11692, t11697, t15563, t15743, t3490, t15239, t486, t11698, t15569, t15498, t3523, t11734, t1174, t11774, t11863, t1216, t15637, t3440, t3515, t44932, t4582, t4984, t5005, t5024, t50857, t50861);
        let (t52674, t52680, t52682, t52684, t52687) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2598(t15495, t3572, t1227, t1653, t248, t45293, t15591, t15643, t3490, t1089, t3507, t607);
    (t52627, t52639, t52659, t52668, t52674, t52680, t52682, t52684, t52687)
}
