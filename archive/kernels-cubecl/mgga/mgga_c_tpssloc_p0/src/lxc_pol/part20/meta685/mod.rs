//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta685 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2595;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2596;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2597;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2598;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta685<F: Float>(t1227: F, t49850: F, t4988: F, t15568: F, t3604: F, t11697: F, t15473: F, t3577: F, t11698: F, t15740: F, t10401: F, t15567: F, t3610: F, t11154: F, t11668: F, t11680: F, t11688: F, t11825: F, t11863: F, t15453: F, t15569: F, t1735: F, t3580: F, t44996: F, t4582: F, t48554: F, t4954: F, t4989: F, t5024: F, t11692: F, t15563: F, t15743: F, t3490: F, t15239: F, t486: F, t15498: F, t3523: F, t11734: F, t1174: F, t11774: F, t1216: F, t15637: F, t3440: F, t3515: F, t44932: F, t4984: F, t5005: F, t50857: F, t50861: F, t15495: F, t3572: F, t1653: F, t248: F, t45293: F, t15591: F, t15643: F, t1089: F, t3507: F, t607: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t52610, t52615, t52619, t52621, t52627) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2595::<F>(t1227, t49850, t4988, t15568, t3604, t11697, t15473, t3577, t11698, t15740, t10401, t15567);
        let t52639 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2596::<F>(t3610, t52627, t11154, t11668, t11680, t11688, t11825, t11863, t1227, t15453, t15569, t1735, t3577, t3580, t44996, t4582, t48554, t4954, t4989, t5024, t52610, t52615, t52619, t52621);
        let (t52659, t52668) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2597::<F>(t11692, t11697, t15563, t15743, t3490, t15239, t486, t11698, t15569, t15498, t3523, t11734, t1174, t11774, t11863, t1216, t15637, t3440, t3515, t44932, t4582, t4984, t5005, t5024, t50857, t50861);
        let (t52674, t52680, t52682, t52684, t52687) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2598::<F>(t15495, t3572, t1227, t1653, t248, t45293, t15591, t15643, t3490, t1089, t3507, t607);
    (t52627, t52639, t52659, t52668, t52674, t52680, t52682, t52684, t52687)
}
