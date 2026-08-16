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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

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
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta813(t12606: f64, t4342: f64, t123: f64, t882: f64, t47787: f64, t59727: f64, t59732: f64, t59735: f64, t59738: f64, t59744: f64, t59748: f64, t59753: f64, t59757: f64, t59759: f64, t59761: f64, t59765: f64, t324: f64, t59650: f64, t59686: f64, t59723: f64, t41656: f64, t47705: f64, t47707: f64, t47709: f64, t47711: f64, t47713: f64, t47715: f64, t47717: f64, t47724: f64, t47730: f64, t47732: f64, t47738: f64, t41658: f64, t41675: f64, t41684: f64, t59655: f64, t59657: f64, t59661: f64, t59663: f64, t59665: f64, t59670: f64, t59674: f64, t59678: f64, t59680: f64, t59684: f64, t42245: f64, t59688: f64, t59692: f64, t59694: f64, t59698: f64, t59700: f64, t59702: f64, t59704: f64, t59708: f64, t59713: f64, t59717: f64, t59721: f64, t41655: f64, t291: f64, t17297: f64, t2932: f64, t2860: f64, t5737: f64, t10756: f64, t10771: f64, t10825: f64, t13716: f64, t14263: f64, t14337: f64, t14366: f64, t14370: f64, t14425: f64, t14453: f64, t14456: f64, t1581: f64, t17366: f64, t17496: f64, t17500: f64, t2863: f64, t2880: f64, t2905: f64, t2906: f64, t2924: f64, t2930: f64, t311: f64, t41821: f64, t49099: f64, t49104: f64, t49422: f64, t5762: f64, t5775: f64, t5794: f64, t59637: f64, t950: f64, t10813: f64, t5758: f64, t17195: f64, t2837: f64, t2841: f64, t5689: f64, t2845: f64, t17471: f64, t923: f64, t1557: f64, t49483: f64, t13515: f64, t4396: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t59767, t59769) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2857(t12606, t4342, t123, t882);
        let t59771 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2858(t47787, t59727, t59732, t59735, t59738, t59744, t59748, t59753, t59757, t59759, t59761, t59765, t59769);
        let (t59774, t59788) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2859(t324, t59650, t59686, t59723, t59771, t41656, t47705, t47707, t47709, t47711, t47713, t47715, t47717, t47724, t47730, t47732, t47738);
        let t59802 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2860(t41658, t41675, t41684, t59655, t59657, t59661, t59663, t59665, t59670, t59674, t59678, t59680, t59684);
        let t59815 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2861(t42245, t59688, t59692, t59694, t59698, t59700, t59702, t59704, t59708, t59713, t59717, t59721);
        let t59829 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2862(t47787, t59727, t59732, t59735, t59738, t59744, t59748, t59753, t59757, t59759, t59761, t59765, t59769);
        let t59846 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2863(t41656, t47705, t47707, t47709, t47711, t47713, t47715, t47717, t47724, t47730, t47732, t47738);
        let t59860 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2864(t41658, t41675, t41684, t59655, t59657, t59661, t59663, t59665, t59670, t59674, t59678, t59680, t59684);
        let t59873 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2865(t41655, t59688, t59692, t59694, t59698, t59700, t59702, t59704, t59708, t59713, t59717, t59721);
        let t59887 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2866(t47787, t59727, t59732, t59735, t59738, t59744, t59748, t59753, t59757, t59759, t59761, t59765, t59769);
        let (t59891, t59928) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2867(t291, t59846, t59860, t59873, t59887, t17297, t2932, t2860, t5737, t10756, t10771, t10825, t13716, t14263, t14337, t14366, t14370, t14425, t14453, t14456, t1581, t17366, t17496, t17500, t2863, t2880, t2905, t2906, t2924, t2930, t311, t41821, t49099, t49104, t49422, t5762, t5775, t5794, t59637, t59774, t59788, t59802, t59815, t59829, t950);
        let (t59941, t59958, t59961, t59962, t59966, t59968) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2868(t10813, t5758, t17195, t2837, t2841, t5689, t2845, t17471, t923, t1557, t49483, t13515, t4396);
    (t59767, t59769, t59774, t59891, t59928, t59941, t59958, t59961, t59962, t59966, t59968)
}
