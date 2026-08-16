//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta468 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1374;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1375;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1376;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1377;
use chunk4::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1378;
use chunk5::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1379;
use chunk6::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1380;
use chunk7::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1381;
use chunk8::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1382;
use chunk9::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1383;
use chunk10::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1384;
use chunk11::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1385;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta468(t2792: f64, t76998: f64, t913: f64, t10632: f64, t41825: f64, t76637: f64, t959: f64, t5742: f64, t48103: f64, t68442: f64, t68444: f64, t68446: f64, t68448: f64, t68452: f64, t68454: f64, t68494: f64, t68498: f64, t68500: f64, t77028: f64, t77030: f64, t77032: f64, t77034: f64, t59657: f64, t60168: f64, t60173: f64, t60204: f64, t68502: f64, t68504: f64, t68506: f64, t76877: f64, t76880: f64, t76887: f64, t76890: f64, t77042: f64, t77073: f64, t77076: f64, t42212: f64, t59688: f64, t59694: f64, t76574: f64, t76578: f64, t76583: f64, t76591: f64, t76599: f64, t76614: f64, t76622: f64, t76893: f64, t76896: f64, t76909: f64, t76915: f64, t42213: f64, t47787: f64, t76587: f64, t76595: f64, t76610: f64, t76618: f64, t76626: f64, t76899: f64, t76903: f64, t76906: f64, t76912: f64, t77102: f64, t77105: f64, t77107: f64, t5758: f64, t10756: f64, t10771: f64, t10813: f64, t10828: f64, t14337: f64, t17355: f64, t21198: f64, t21207: f64, t21239: f64, t21242: f64, t21247: f64, t2886: f64, t2888: f64, t2905: f64, t2932: f64, t42111: f64, t42113: f64, t42154: f64, t42226: f64, t42228: f64, t4449: f64, t49099: f64, t49104: f64, t49285: f64, t5775: f64, t5791: f64, t5794: f64, t60343: f64, t60424: f64, t77139: f64, t924: f64, t932: f64, t951: f64, t10811: f64, t1569: f64, t1581: f64, t17428: f64, t21115: f64, t21195: f64, t2861: f64, t41826: f64, t4411: f64, t49430: f64, t5743: f64, t5759: f64, t5762: f64, t59920: f64, t60407: f64, t69047: f64, t69182: f64, t76647: f64, t76652: f64, t76654: f64, t76657: f64, t76659: f64, t76661: f64, t77220: f64, t943: f64, t14271: f64, t1568: f64, t17499: f64, t17547: f64, t21194: f64, t21306: f64, t5790: f64, t69380: f64, t76632: f64, t76663: f64, t76665: f64, t76668: f64, t76671: f64, t77001: f64, t77006: f64, t77119: f64, t77124: f64, t77127: f64, t77130: f64, t42245: f64, t41655: f64, t291: f64, t14263: f64, t14276: f64, t1580: f64, t21238: f64, t21259: f64, t21309: f64, t21312: f64, t21321: f64, t2930: f64, t311: f64, t59941: f64, t69276: f64, t77133: f64, t77135: f64, t77138: f64, t77226: f64, t77229: f64, t300: f64, t10629: f64, t2929: f64, t77153: f64, t77157: f64, t77159: f64, t77224: f64, t76995: f64, t77017: f64, t77151: f64, t1020: f64, t1021: f64, t1041: f64, t1044: f64, t1618: f64, t17607: f64, t21580: f64, t248: f64, t3062: f64, t3131: f64, t360: f64, t369: f64, t378: f64, t42347: f64, t43317: f64, t4644: f64, t5880: f64, t5900: f64, t61739: f64, t68: f64, t70148: f64, t70162: f64, t70166: f64, t70199: f64, t70209: f64, t70214: f64, t70227: f64, t75836: f64, t76597: f64, t76612: f64, t76620: f64, t76740: f64, t76977: f64, t973: f64, t974: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t77232, t77236, t77239, t77257) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1374(t2792, t76998, t913, t10632, t41825, t76637, t959, t5742, t48103, t68442, t68444, t68446, t68448, t68452, t68454, t68494, t68498, t68500, t77028, t77030, t77032, t77034);
        let t77272 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1375(t59657, t60168, t60173, t60204, t68502, t68504, t68506, t76877, t76880, t76887, t76890, t77042, t77073, t77076);
        let t77287 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1376(t42212, t59688, t59694, t76574, t76578, t76583, t76591, t76599, t76614, t76622, t76893, t76896, t76909, t76915);
        let t77301 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1377(t42213, t47787, t76587, t76595, t76610, t76618, t76626, t76899, t76903, t76906, t76912, t77102, t77105, t77107);
        let (t77328, t77343) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1378(t5758, t10756, t10771, t10813, t10828, t14337, t17355, t21198, t21207, t21239, t21242, t21247, t2886, t2888, t2905, t2932, t42111, t42113, t42154, t42226, t42228, t4449, t49099, t49104, t49285, t5775, t5791, t5794, t60343, t60424, t76637, t77139, t77239, t77257, t77272, t77287, t77301, t924, t932, t951);
        let t77370 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1379(t10632, t10811, t1569, t1581, t17428, t21115, t21195, t2861, t2888, t41826, t4411, t49430, t5743, t5759, t5762, t59920, t60407, t69047, t69182, t76637, t76647, t76652, t76654, t76657, t76659, t76661, t77220, t77239, t77328, t932, t943, t951);
        let t77390 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1380(t10756, t10771, t14271, t1568, t1569, t17499, t17547, t21194, t21306, t2861, t2886, t5742, t5743, t5758, t5790, t69380, t76632, t76663, t76665, t76668, t76671, t77001, t77006, t77119, t77124, t77127, t77130);
        let (t77427, t77440) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1381(t42245, t47787, t59657, t68442, t76574, t76578, t76583, t76587, t76591, t76595, t76599, t59688, t59694, t68444, t68446, t68448, t68494, t68498, t76610, t76614, t76618, t76622, t76626);
        let (t77454, t77467) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1382(t41655, t47787, t59657, t68442, t76574, t76578, t76583, t76587, t76591, t76595, t76599, t59688, t59694, t68444, t68446, t68448, t68494, t68498, t76610, t76614, t76618, t76622, t76626);
        let (t77470, t77471) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1383(t291, t77454, t77467, t10811, t10828, t14263, t14271, t14276, t14337, t1580, t1581, t21238, t21259, t21309, t21312, t21321, t2905, t2930, t2932, t311, t5742, t5775, t5790, t5794, t59941, t69276, t77133, t77135, t77138, t77139, t77226, t77229, t77232, t77427, t77440);
        let (t77474, t77478, t77482, t77483) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1384(t300, t77343, t77370, t77390, t77471, t10629, t2932, t76637, t959, t2929, t77139, t77153, t77157, t77159, t77224, t77226, t77229, t77232, t77236, t77470);
        let (t77485, t77498) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1385(t76995, t77017, t77151, t77483, t1020, t1021, t1041, t1044, t1618, t17607, t21580, t248, t3062, t3131, t360, t369, t378, t42347, t43317, t4644, t5880, t5900, t61739, t68, t70148, t70162, t70166, t70199, t70209, t70214, t70227, t75836, t76597, t76612, t76620, t76740, t76977, t973, t974);
    (t77232, t77236, t77470, t77474, t77478, t77482, t77485, t77498)
}
