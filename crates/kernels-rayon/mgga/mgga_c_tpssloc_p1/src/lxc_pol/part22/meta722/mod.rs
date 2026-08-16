//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta722 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2356;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2357;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2358;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2359;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2360;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2361;
use chunk6::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2362;
use chunk7::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2363;
use chunk8::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2364;
use chunk9::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2365;
use chunk10::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2366;
use chunk11::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2367;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta722(t13065: f64, t13176: f64, t13384: f64, t13390: f64, t13397: f64, t13433: f64, t1528: f64, t16673: f64, t16753: f64, t16758: f64, t16759: f64, t16815: f64, t16816: f64, t16820: f64, t16823: f64, t16830: f64, t17030: f64, t17031: f64, t17034: f64, t17037: f64, t17041: f64, t17046: f64, t17050: f64, t17057: f64, t17064: f64, t17090: f64, t17092: f64, t20867: f64, t20870: f64, t20871: f64, t20873: f64, t20986: f64, t21014: f64, t21028: f64, t21033: f64, t21050: f64, t226: f64, t235: f64, t25093: f64, t255: f64, t2597: f64, t2617: f64, t2718: f64, t2732: f64, t4162: f64, t4166: f64, t4182: f64, t4234: f64, t4268: f64, t4273: f64, t4280: f64, t4281: f64, t4283: f64, t4288: f64, t4290: f64, t4291: f64, t4292: f64, t4295: f64, t4301: f64, t47374: f64, t47386: f64, t5575: f64, t5585: f64, t5617: f64, t5637: f64, t5648: f64, t5653: f64, t5655: f64, t59498: f64, t59519: f64, t67339: f64, t67344: f64, t67350: f64, t67358: f64, t67392: f64, t67403: f64, t67405: f64, t67429: f64, t67582: f64, t67596: f64, t68144: f64, t68211: f64, t68217: f64, t68256: f64, t68299: f64, t808: f64, t812: f64, t829: f64, t855: f64, t858: f64, t865: f64, t866: f64, t21064: f64, t225: f64, t13042: f64, t13463: f64, t17052: f64, t17070: f64, t21034: f64, t252: f64, t259: f64, t2713: f64, t4142: f64, t4147: f64, t5631: f64, t5658: f64, t59503: f64, t68143: f64, t13053: f64, t1492: f64, t1519: f64, t16804: f64, t17022: f64, t17056: f64, t20936: f64, t218: f64, t25168: f64, t4265: f64, t46488: f64, t5558: f64, t58143: f64, t852: f64, t262: f64, t5527: f64, t193: f64, t202: f64, t39585: f64, t39590: f64, t4119: f64, t67322: f64, t67457: f64, t67458: f64, t67461: f64, t67464: f64, t67466: f64, t67472: f64, t67475: f64, t870: f64, t1530: f64, t16596: f64, t16944: f64, t17120: f64, t1877: f64, t2522: f64, t41258: f64, t41262: f64, t4310: f64, t4314: f64, t46436: f64, t59584: f64, t67487: f64, t67488: f64, t67489: f64, t67490: f64, t67494: f64, t12899: f64, t16662: f64, t20753: f64, t20769: f64, t20778: f64, t39658: f64, t40772: f64, t4315: f64, t46341: f64, t46438: f64, t5544: f64, t67495: f64, t67496: f64, t67497: f64, t67498: f64, t868: f64, t16625: f64, t25374: f64, t4255: f64, t59564: f64, t67499: f64, t67500: f64, t67501: f64, t67502: f64, t67503: f64, t67504: f64, t67506: f64, t67507: f64, t67508: f64, t1484: f64, t16606: f64, t16949: f64, t17116: f64, t20800: f64, t2523: f64, t25365: f64, t39249: f64, t39256: f64, t39373: f64, t39397: f64, t39400: f64, t39408: f64, t39463: f64, t39468: f64, t39472: f64, t39476: f64, t39529: f64, t39593: f64, t40708: f64, t40721: f64, t40779: f64, t40784: f64, t41254: f64, t46138: f64, t46218: f64, t46235: f64, t46336: f64, t57932: f64, t67044: f64, t67086: f64, t67087: f64, t67088: f64, t67089: f64, t67090: f64, t67101: f64, t67104: f64, t67105: f64, t67112: f64, t67134: f64, t67137: f64, t67141: f64, t67160: f64, t67175: f64, t67195: f64, t67204: f64, t67206: f64, t67207: f64, t67210: f64, t67211: f64, t67212: f64, t67218: f64, t67286: f64, t67478: f64, t67480: f64, t67482: f64, t67484: f64, t67485: f64, t67486: f64, t776: f64, t2: f64, t5669: f64, t584: f64, t21589: f64, t2940: f64, t21152: f64, t690: f64, t21155: f64, t21146: f64, t21149: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let t68305 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2356(t13065, t13176, t13384, t13390, t13397, t13433, t1528, t16673, t16753, t16758, t16759, t16815, t16816, t16820, t16823, t16830, t17030, t17031, t17034, t17037, t17041, t17046, t17050, t17057, t17064, t17090, t17092, t20867, t20870, t20871, t20873, t20986, t21014, t21028, t21033, t21050, t226, t235, t25093, t255, t2597, t2617, t2718, t2732, t4162, t4166, t4182, t4234, t4268, t4273, t4280, t4281, t4283, t4288, t4290, t4291, t4292, t4295, t4301, t47374, t47386, t5575, t5585, t5617, t5637, t5648, t5653, t5655, t59498, t59519, t67339, t67344, t67350, t67358, t67392, t67403, t67405, t67429, t67582, t67596, t68144, t68211, t68217, t68256, t68299, t808, t812, t829, t855, t858, t865, t866);
        let t68333 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2357(t21064, t225, t13042, t13463, t1528, t17050, t17052, t17070, t21034, t252, t259, t2713, t4142, t4147, t4268, t4273, t4301, t5631, t5637, t5658, t59503, t68143, t866);
        let t68365 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2358(t13042, t13053, t13065, t1492, t1519, t1528, t16804, t17022, t17056, t17090, t20936, t21034, t21050, t218, t25168, t259, t2597, t2713, t4265, t4301, t46488, t5558, t5637, t5658, t58143, t68211, t852);
        let t68375 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2359(t262, t5527, t193, t202, t39585, t39590, t4119, t67322, t67457, t67458, t67461, t67464, t67466, t67472, t67475, t68305, t68333, t68365, t870);
        let t68391 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2360(t1530, t16596, t16944, t17120, t1877, t2522, t41258, t41262, t4310, t4314, t46436, t59584, t67487, t67488, t67489, t67490, t67494);
        let t68407 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2361(t12899, t16662, t1877, t20753, t20769, t20778, t39658, t40772, t4314, t4315, t46341, t46438, t5544, t67495, t67496, t67497, t67498, t868);
        let t68414 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2362(t16625, t1877, t25374, t4255, t4314, t59564, t67499, t67500, t67501, t67502, t67503, t67504, t67506, t67507, t67508);
        let t68418 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2363(t1484, t16606, t16625, t16949, t17116, t20800, t2522, t2523, t25365, t39249, t39256, t39373, t39397, t39400, t39408, t39463, t39468, t39472, t39476, t39529, t39593, t40708, t40721, t40779, t40784, t4119, t41254, t4310, t4314, t46138, t46218, t46235, t46336, t57932, t67044, t67086, t67087, t67088, t67089, t67090, t67101, t67104, t67105, t67112, t67134, t67137, t67141, t67160, t67175, t67195, t67204, t67206, t67207, t67210, t67211, t67212, t67218, t67286, t67478, t67480, t67482, t67484, t67485, t67486, t68375, t68391, t68407, t68414, t776);
        let (t68427, t68441, t68442) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2364(t2, t5669, t584, t21589, t2940, t21152, t690);
        let t68444 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2365(t21155, t690);
        let t68446 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2366(t21146, t690);
        let t68448 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2367(t21149, t690);
    (t68418, t68427, t68441, t68442, t68444, t68446, t68448)
}
