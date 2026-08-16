//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta813 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;
mod chunk9;
mod chunk10;
mod chunk11;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2857;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2858;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2859;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2860;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2861;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2862;
use chunk6::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2863;
use chunk7::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2864;
use chunk8::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2865;
use chunk9::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2866;
use chunk10::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2867;
use chunk11::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2868;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta813<F: Float>(t12606: F, t4342: F, t123: F, t882: F, t47787: F, t59727: F, t59732: F, t59735: F, t59738: F, t59744: F, t59748: F, t59753: F, t59757: F, t59759: F, t59761: F, t59765: F, t324: F, t59650: F, t59686: F, t59723: F, t41656: F, t47705: F, t47707: F, t47709: F, t47711: F, t47713: F, t47715: F, t47717: F, t47724: F, t47730: F, t47732: F, t47738: F, t41658: F, t41675: F, t41684: F, t59655: F, t59657: F, t59661: F, t59663: F, t59665: F, t59670: F, t59674: F, t59678: F, t59680: F, t59684: F, t42245: F, t59688: F, t59692: F, t59694: F, t59698: F, t59700: F, t59702: F, t59704: F, t59708: F, t59713: F, t59717: F, t59721: F, t41655: F, t291: F, t17297: F, t2932: F, t2860: F, t5737: F, t10756: F, t10771: F, t10825: F, t13716: F, t14263: F, t14337: F, t14366: F, t14370: F, t14425: F, t14453: F, t14456: F, t1581: F, t17366: F, t17496: F, t17500: F, t2863: F, t2880: F, t2905: F, t2906: F, t2924: F, t2930: F, t311: F, t41821: F, t49099: F, t49104: F, t49422: F, t5762: F, t5775: F, t5794: F, t59637: F, t950: F, t10813: F, t5758: F, t17195: F, t2837: F, t2841: F, t5689: F, t2845: F, t17471: F, t923: F, t1557: F, t49483: F, t13515: F, t4396: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t59767, t59769) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2857::<F>(t12606, t4342, t123, t882);
        let t59771 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2858::<F>(t47787, t59727, t59732, t59735, t59738, t59744, t59748, t59753, t59757, t59759, t59761, t59765, t59769);
        let (t59774, t59788) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2859::<F>(t324, t59650, t59686, t59723, t59771, t41656, t47705, t47707, t47709, t47711, t47713, t47715, t47717, t47724, t47730, t47732, t47738);
        let t59802 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2860::<F>(t41658, t41675, t41684, t59655, t59657, t59661, t59663, t59665, t59670, t59674, t59678, t59680, t59684);
        let t59815 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2861::<F>(t42245, t59688, t59692, t59694, t59698, t59700, t59702, t59704, t59708, t59713, t59717, t59721);
        let t59829 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2862::<F>(t47787, t59727, t59732, t59735, t59738, t59744, t59748, t59753, t59757, t59759, t59761, t59765, t59769);
        let t59846 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2863::<F>(t41656, t47705, t47707, t47709, t47711, t47713, t47715, t47717, t47724, t47730, t47732, t47738);
        let t59860 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2864::<F>(t41658, t41675, t41684, t59655, t59657, t59661, t59663, t59665, t59670, t59674, t59678, t59680, t59684);
        let t59873 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2865::<F>(t41655, t59688, t59692, t59694, t59698, t59700, t59702, t59704, t59708, t59713, t59717, t59721);
        let t59887 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2866::<F>(t47787, t59727, t59732, t59735, t59738, t59744, t59748, t59753, t59757, t59759, t59761, t59765, t59769);
        let (t59891, t59928) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2867::<F>(t291, t59846, t59860, t59873, t59887, t17297, t2932, t2860, t5737, t10756, t10771, t10825, t13716, t14263, t14337, t14366, t14370, t14425, t14453, t14456, t1581, t17366, t17496, t17500, t2863, t2880, t2905, t2906, t2924, t2930, t311, t41821, t49099, t49104, t49422, t5762, t5775, t5794, t59637, t59774, t59788, t59802, t59815, t59829, t950);
        let (t59941, t59958, t59961, t59962, t59966, t59968) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2868::<F>(t10813, t5758, t17195, t2837, t2841, t5689, t2845, t17471, t923, t1557, t49483, t13515, t4396);
    (t59767, t59769, t59774, t59891, t59928, t59941, t59958, t59961, t59962, t59966, t59968)
}
