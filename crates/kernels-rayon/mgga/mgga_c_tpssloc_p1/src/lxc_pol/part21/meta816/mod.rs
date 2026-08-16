//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta816 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2873;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2874;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2875;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2876;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2877;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2878;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta816(t10702: f64, t2793: f64, t5730: f64, t13654: f64, t1557: f64, t2792: f64, t10661: f64, t2836: f64, t17527: f64, t42028: f64, t41831: f64, t47705: f64, t47707: f64, t47709: f64, t47711: f64, t47713: f64, t47715: f64, t47717: f64, t47724: f64, t47730: f64, t47732: f64, t48087: f64, t48096: f64, t48098: f64, t48140: f64, t48143: f64, t55716: f64, t41656: f64, t47738: f64, t41658: f64, t41675: f64, t41684: f64, t59655: f64, t59657: f64, t59661: f64, t59663: f64, t59665: f64, t59670: f64, t59674: f64, t59678: f64, t59680: f64, t59684: f64, t41904: f64, t59688: f64, t59692: f64, t59694: f64, t59698: f64, t59700: f64, t59702: f64, t59704: f64, t59708: f64, t59713: f64, t59717: f64, t59721: f64, t47787: f64, t59727: f64, t59732: f64, t59735: f64, t59738: f64, t59744: f64, t59748: f64, t59753: f64, t59757: f64, t59759: f64, t59761: f64, t59765: f64, t59769: f64, t901: f64, t42444: f64, t43317: f64, t41863: f64, t41870: f64, t41872: f64, t48103: f64, t48116: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t60047, t60050, t60053, t60056, t60079) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2873(t10702, t2793, t5730, t13654, t1557, t2792, t10661, t2836, t17527, t42028, t41831, t47705, t47707, t47709, t47711, t47713, t47715, t47717, t47724, t47730, t47732, t48087, t48096, t48098);
        let (t60091, t60106) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2874(t48140, t48143, t55716, t41656, t47705, t47707, t47709, t47711, t47713, t47715, t47717, t47724, t47730, t47732, t47738);
        let t60120 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2875(t41658, t41675, t41684, t59655, t59657, t59661, t59663, t59665, t59670, t59674, t59678, t59680, t59684);
        let t60133 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2876(t41904, t59688, t59692, t59694, t59698, t59700, t59702, t59704, t59708, t59713, t59717, t59721);
        let t60147 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2877(t47787, t59727, t59732, t59735, t59738, t59744, t59748, t59753, t59757, t59759, t59761, t59765, t59769);
        let (t60149, t60150, t60153, t60156, t60158) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2878(t60106, t60120, t60133, t60147, t901, t42444, t48140, t55716, t43317, t41656, t41658, t41675, t41684, t41863, t41870, t41872, t47738, t48103, t48116, t59655, t60091);
    (t60047, t60050, t60053, t60056, t60079, t60091, t60149, t60150, t60153, t60156, t60158)
}
