//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta331 (260520-c91 hierarchical CSE).
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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1181;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1182;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1183;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1184;
use chunk4::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1185;
use chunk5::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1186;
use chunk6::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1187;
use chunk7::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1188;
use chunk8::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1189;
use chunk9::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1190;
use chunk10::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1191;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta331(t40419: f64, t535: f64, t9538: f64, t12231: f64, t3726: f64, t12199: f64, t12208: f64, t118: f64, t12012: f64, t3739: f64, t794: f64, t12217: f64, t40021: f64, t1315: f64, t210: f64, t214: f64, t39892: f64, t40025: f64, t40026: f64, t40401: f64, t40402: f64, t40404: f64, t40407: f64, t40410: f64, t40415: f64, t40389: f64, t225: f64, t3774: f64, t3862: f64, t241: f64, t6597: f64, t248: f64, t555: f64, t557: f64, t12368: f64, t12369: f64, t12402: f64, t12407: f64, t12419: f64, t12420: f64, t12422: f64, t12426: f64, t12429: f64, t1352: f64, t16233: f64, t16305: f64, t3803: f64, t3805: f64, t3807: f64, t40183: f64, t40197: f64, t40304: f64, t40329: f64, t40335: f64, t5246: f64, t5248: f64, t5250: f64, t554: f64, t559: f64, t39970: f64, t40010: f64, t40062: f64, t40101: f64, t40147: f64, t40204: f64, t40303: f64, t12167: f64, t562: f64, t12434: f64, t1338: f64, t3787: f64, t3879: f64, t12248: f64, t1372: f64, t12169: f64, t12171: f64, t12178: f64, t12244: f64, t12255: f64, t12259: f64, t12260: f64, t12435: f64, t1332: f64, t1336: f64, t22694: f64, t3773: f64, t3777: f64, t3851: f64, t3856: f64, t3901: f64, t3909: f64, t5344: f64, t544: f64, t553: f64, t40041: f64, t12168: f64, t12172: f64, t12179: f64, t12240: f64, t12241: f64, t12256: f64, t12267: f64, t12273: f64, t1380: f64, t16033: f64, t16047: f64, t16055: f64, t22740: f64, t3905: f64, t40271: f64, t5334: f64, t564: f64, t12019: f64, t566: f64, t68: f64, t3888: f64, t12023: f64, t12027: f64, t12030: f64, t12033: f64, t12181: f64, t12237: f64, t12238: f64, t12249: f64, t12251: f64, t12252: f64, t12438: f64, t1323: f64, t1375: f64, t1378: f64, t1381: f64, t1383: f64, t3752: f64, t3758: f64, t3793: f64, t3882: f64, t3889: f64, t3897: f64, t3898: f64, t3902: f64, t3907: f64, t39938: f64, t40047: f64, t40118: f64, t40133: f64, t40148: f64, t40153: f64, t40162: f64, t539: f64, t568: f64, t3698: f64, t3700: f64, t1297: f64, t1390: f64, t16490: f64, t193: f64, t3719: f64, t39852: f64, t39854: f64, t39856: f64, t39858: f64, t39932: f64, t40222: f64, t40224: f64, t40226: f64, t40228: f64, t40230: f64, t40232: f64, t40234: f64, t533: f64, t40: f64, t10121: f64, t870: f64, t2517: f64, t2519: f64, t195: f64, t632: f64, t2244: f64, t2250: f64, t2433: f64, t39097: f64, t39103: f64, t39110: f64, t73: f64, t9258: f64, t9427: f64, t9430: f64, zeta_threshold: f64, t52: f64, t197: f64, t636: f64, t2440: f64, t76: f64, t9438: f64, t9441: f64, t145: f64, t185: f64, t2531: f64, t9892: f64, t67: f64, t758: f64, t9915: f64, t10126: f64, t2379: f64, t2522: f64, t2523: f64, t39249: f64, t39256: f64, t39309: f64, t39312: f64, t39316: f64, t39320: f64, t4314: f64, t776: f64, t9516: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t40422, t40423, t40425, t40429, t40431) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1181(t40419, t535, t9538, t12231, t3726, t12199, t12208, t118, t12012, t3739, t794, t12217, t40021);
        let t40437 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1182(t1315, t210, t214, t39892, t40025, t40026, t40401, t40402, t40404, t40407, t40410, t40415, t40422, t40423, t40425, t40429, t40431);
        let (t40438, t40439, t40445, t40450) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1183(t40389, t40437, t225, t3774, t3862, t241, t6597, t248, t555, t557, t12368, t12369, t12402, t12407, t12419, t12420, t12422, t12426, t12429, t1352, t16233, t16305, t3803, t3805, t3807, t40183, t40197, t40304, t40329, t40335, t5246, t5248, t5250, t554, t559);
        let (t40453, t40475, t40479) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1184(t39970, t40010, t40062, t40101, t40147, t40204, t40303, t40450, t12167, t562, t12434, t1338);
        let (t40486, t40492, t40524) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1185(t3787, t3879, t12248, t1372, t12169, t12171, t12178, t12244, t12255, t12259, t12260, t12435, t1332, t1336, t1352, t22694, t3773, t3777, t3851, t3856, t3901, t3909, t40453, t40475, t5344, t544, t553);
        let (t40541, t40576) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1186(t40041, t562, t12168, t12172, t12179, t12240, t12241, t12256, t12267, t12273, t1336, t1380, t16033, t16047, t16055, t22740, t3777, t3901, t3905, t40271, t40335, t40439, t5334, t564);
        let t40603 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1187(t12019, t566, t68, t3888, t12023, t12027, t12030, t12033, t12181, t12237, t12238, t12240, t12249, t12251, t12252, t12259, t12267, t12434, t12438, t1323, t1336, t1352, t1372, t1375, t1378, t1380, t1381, t1383, t22694, t22740, t3752, t3758, t3777, t3793, t3851, t3879, t3882, t3889, t3897, t3898, t3902, t3907, t39938, t40047, t40118, t40133, t40148, t40153, t40162, t40438, t40453, t40475, t40479, t40486, t40492, t40524, t40541, t40576, t5250, t5334, t5344, t539, t562, t568);
        let t40615 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1188(t3698, t3700, t1297, t1390, t16490, t193, t3719, t39852, t39854, t39856, t39858, t39892, t39932, t40222, t40224, t40226, t40228, t40230, t40232, t40234, t40603, t533);
        let (t40622, t40627, t40645) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1189(t40, t10121, t870, t2517, t2519, t195, t632, t2244, t2250, t2433, t39097, t39103, t39110, t73, t9258, t9427, t9430, zeta_threshold);
        let t40660 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1190(t52, t197, t636, t2244, t2250, t2440, t39097, t39103, t39110, t76, t9258, t9438, t9441, zeta_threshold);
        let (t40661, t40663, t40668, t40671, t40672) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1191(t40645, t40660, t145, t185, t2531, t9892, t67, t758, t9915, t10126, t2379, t2522, t2523, t39249, t39256, t39309, t39312, t39316, t39320, t40622, t40627, t4314, t776, t9516);
    (t40445, t40615, t40627, t40661, t40663, t40668, t40671, t40672)
}
