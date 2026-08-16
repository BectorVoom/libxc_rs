//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta634 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2323;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2324;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2325;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2326;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2327;
use chunk5::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2328;
use chunk6::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2329;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta634(t13580: f64, t690: f64, t47681: f64, t47686: f64, t47691: f64, t47695: f64, t47699: f64, t47703: f64, t47706: f64, t47707: f64, t47709: f64, t47711: f64, t47713: f64, t47715: f64, t47717: f64, t47722: f64, t47724: f64, t47728: f64, t47731: f64, t47732: f64, t47736: f64, t1409: f64, t41687: f64, t9288: f64, t10564: f64, t123: f64, t13554: f64, t882: f64, t12606: f64, t2775: f64, t607: f64, t13541: f64, t2250: f64, t4342: f64, t9258: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t47738 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2323(t13580, t690);
        let t47740 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2324(t47681, t47686, t47691, t47695, t47699, t47703, t47706, t47707, t47709, t47711, t47713, t47715, t47717, t47722, t47724, t47728, t47731, t47732, t47736, t47738);
        let (t47742, t47744) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2325(t1409, t41687, t9288, t10564, t123);
        let (t47746, t47748) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2326(t13554, t9288, t123, t882);
        let (t47759, t47761) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2327(t12606, t2775, t607, t123, t882);
        let (t47763, t47765) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2328(t13541, t2250, t123, t882);
        let (t47767, t47769) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2329(t4342, t9258, t123, t882);
    (t47738, t47740, t47742, t47744, t47746, t47748, t47759, t47761, t47763, t47765, t47767, t47769)
}
