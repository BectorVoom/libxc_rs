//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta681 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2567;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2568;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2569;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2570;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2571;
use chunk5::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2572;
use chunk6::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2573;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta681(t1254: f64, t3633: f64, t1157: f64, t1164: f64, t14829: f64, t3375: f64, t14966: f64, t3378: f64, t15823: f64, t225: f64, t15800: f64, t15808: f64, t11598: f64, t11919: f64, t11935: f64, t1238: f64, t1251: f64, t1252: f64, t14972: f64, t15786: f64, t15794: f64, t15797: f64, t15803: f64, t15820: f64, t1751: f64, t1761: f64, t3487: f64, t3598: f64, t3600: f64, t3631: f64, t44412: f64, t4945: f64, t498: f64, t14731: f64, t15419: f64, t3447: f64, t12606: f64, t3450: f64, t1714: f64, t44583: f64, t3451: f64, t458: f64, t44584: f64, t4904: f64, t44510: f64, t14753: f64, t15402: f64, t11509: f64, t11566: f64, t11576: f64, t11580: f64, t11585: f64, t11594: f64, t15376: f64, t15395: f64, t3449: f64, t4900: f64, t4908: f64, t50879: f64, t50884: f64, t50915: f64, t50929: f64, t14744: f64, t1174: f64, t135: f64, t15359: f64, t11589: f64, t15293: f64, t15382: f64, t44525: f64, t11588: f64, t4928: f64, t15357: f64, t3448: f64, t11579: f64, t11584: f64, t11593: f64, t15313: f64, t15320: f64, t44517: f64, t44536: f64, t44540: f64, t44558: f64, t4919: f64, t50857: f64, t50861: f64, t50873: f64, t50964: f64, t14740: f64, t15338: f64, t461: f64, t4729: f64, t15418: f64, t11571: f64, t14736: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t51906, t51913, t51916, t51925, t51928, t51937) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2567(t1254, t3633, t1157, t1164, t14829, t3375, t14966, t3378, t15823, t225, t15800, t15808);
        let t51946 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2568(t11598, t11919, t11935, t1238, t1251, t1252, t14972, t15786, t15794, t15797, t15803, t15820, t1751, t1761, t3487, t3598, t3600, t3631, t44412, t4945, t498, t51925, t51928, t51937);
        let (t51948, t51961, t51971, t51975, t51981, t51988) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2569(t14731, t15419, t3447, t12606, t3450, t1714, t44583, t3451, t458, t44584, t4904, t44510);
        let t51993 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2570(t14753, t15402, t3447, t11509, t11566, t11576, t11580, t11585, t11594, t15376, t15395, t3449, t4900, t4908, t50879, t50884, t50915, t50929, t51948, t51961, t51971, t51975, t51981, t51988);
        let (t51995, t52013, t52019, t52022, t52036) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2571(t14744, t15402, t3447, t1174, t135, t15359, t11589, t15293, t15382, t44525, t11588, t4928);
        let t52047 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2572(t3447, t3451, t52036, t15357, t3448, t11579, t11584, t11593, t15313, t15320, t15382, t44517, t44536, t44540, t44558, t4900, t4904, t4908, t4919, t50857, t50861, t50873, t50964, t51995, t52013, t52019, t52022);
        let (t52050, t52053, t52058, t52061, t52064) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2573(t14740, t15419, t3447, t11584, t15338, t44583, t461, t4729, t15418, t1714, t11571, t14736);
    (t51906, t51913, t51916, t51946, t51993, t52047, t52050, t52053, t52058, t52061, t52064)
}
