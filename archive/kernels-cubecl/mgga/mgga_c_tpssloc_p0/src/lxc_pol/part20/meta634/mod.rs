//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta634 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2323;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2324;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2325;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2326;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2327;
use chunk5::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2328;
use chunk6::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2329;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta634<F: Float>(t13580: F, t690: F, t47681: F, t47686: F, t47691: F, t47695: F, t47699: F, t47703: F, t47706: F, t47707: F, t47709: F, t47711: F, t47713: F, t47715: F, t47717: F, t47722: F, t47724: F, t47728: F, t47731: F, t47732: F, t47736: F, t1409: F, t41687: F, t9288: F, t10564: F, t123: F, t13554: F, t882: F, t12606: F, t2775: F, t607: F, t13541: F, t2250: F, t4342: F, t9258: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t47738 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2323::<F>(t13580, t690);
        let t47740 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2324::<F>(t47681, t47686, t47691, t47695, t47699, t47703, t47706, t47707, t47709, t47711, t47713, t47715, t47717, t47722, t47724, t47728, t47731, t47732, t47736, t47738);
        let (t47742, t47744) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2325::<F>(t1409, t41687, t9288, t10564, t123);
        let (t47746, t47748) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2326::<F>(t13554, t9288, t123, t882);
        let (t47759, t47761) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2327::<F>(t12606, t2775, t607, t123, t882);
        let (t47763, t47765) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2328::<F>(t13541, t2250, t123, t882);
        let (t47767, t47769) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2329::<F>(t4342, t9258, t123, t882);
    (t47738, t47740, t47742, t47744, t47746, t47748, t47759, t47761, t47763, t47765, t47767, t47769)
}
