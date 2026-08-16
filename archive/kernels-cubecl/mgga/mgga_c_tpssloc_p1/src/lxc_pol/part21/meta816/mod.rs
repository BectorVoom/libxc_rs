//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta816 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2873;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2874;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2875;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2876;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2877;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2878;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta816<F: Float>(t10702: F, t2793: F, t5730: F, t13654: F, t1557: F, t2792: F, t10661: F, t2836: F, t17527: F, t42028: F, t41831: F, t47705: F, t47707: F, t47709: F, t47711: F, t47713: F, t47715: F, t47717: F, t47724: F, t47730: F, t47732: F, t48087: F, t48096: F, t48098: F, t48140: F, t48143: F, t55716: F, t41656: F, t47738: F, t41658: F, t41675: F, t41684: F, t59655: F, t59657: F, t59661: F, t59663: F, t59665: F, t59670: F, t59674: F, t59678: F, t59680: F, t59684: F, t41904: F, t59688: F, t59692: F, t59694: F, t59698: F, t59700: F, t59702: F, t59704: F, t59708: F, t59713: F, t59717: F, t59721: F, t47787: F, t59727: F, t59732: F, t59735: F, t59738: F, t59744: F, t59748: F, t59753: F, t59757: F, t59759: F, t59761: F, t59765: F, t59769: F, t901: F, t42444: F, t43317: F, t41863: F, t41870: F, t41872: F, t48103: F, t48116: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t60047, t60050, t60053, t60056, t60079) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2873::<F>(t10702, t2793, t5730, t13654, t1557, t2792, t10661, t2836, t17527, t42028, t41831, t47705, t47707, t47709, t47711, t47713, t47715, t47717, t47724, t47730, t47732, t48087, t48096, t48098);
        let (t60091, t60106) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2874::<F>(t48140, t48143, t55716, t41656, t47705, t47707, t47709, t47711, t47713, t47715, t47717, t47724, t47730, t47732, t47738);
        let t60120 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2875::<F>(t41658, t41675, t41684, t59655, t59657, t59661, t59663, t59665, t59670, t59674, t59678, t59680, t59684);
        let t60133 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2876::<F>(t41904, t59688, t59692, t59694, t59698, t59700, t59702, t59704, t59708, t59713, t59717, t59721);
        let t60147 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2877::<F>(t47787, t59727, t59732, t59735, t59738, t59744, t59748, t59753, t59757, t59759, t59761, t59765, t59769);
        let (t60149, t60150, t60153, t60156, t60158) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2878::<F>(t60106, t60120, t60133, t60147, t901, t42444, t48140, t55716, t43317, t41656, t41658, t41675, t41684, t41863, t41870, t41872, t47738, t48103, t48116, t59655, t60091);
    (t60047, t60050, t60053, t60056, t60079, t60091, t60149, t60150, t60153, t60156, t60158)
}
