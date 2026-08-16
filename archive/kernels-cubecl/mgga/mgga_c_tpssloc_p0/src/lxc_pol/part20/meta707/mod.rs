//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta707 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2698;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2699;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2700;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2701;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2702;
use chunk5::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2703;
use chunk6::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2704;
use chunk7::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2705;
use chunk8::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2706;
use chunk9::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2707;
use chunk10::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2708;
use chunk11::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2709;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta707<F: Float>(t1338: F, t16413: F, t12168: F, t12181: F, t12238: F, t1332: F, t1336: F, t1352: F, t1380: F, t1381: F, t16052: F, t16055: F, t16060: F, t16206: F, t16414: F, t1825: F, t1840: F, t3901: F, t3907: F, t40479: F, t5234: F, t5348: F, t53909: F, t54527: F, t16463: F, t225: F, t12021: F, t12027: F, t12033: F, t12178: F, t12179: F, t12240: F, t12241: F, t12244: F, t12251: F, t12252: F, t12255: F, t12256: F, t12260: F, t12267: F, t12434: F, t12438: F, t12444: F, t1323: F, t1375: F, t1378: F, t1383: F, t1386: F, t16030: F, t16033: F, t16036: F, t16037: F, t16040: F, t16044: F, t16047: F, t16048: F, t16049: F, t16065: F, t16123: F, t16133: F, t16136: F, t16416: F, t16419: F, t16423: F, t16428: F, t16429: F, t16453: F, t16471: F, t1807: F, t19654: F, t3773: F, t3777: F, t3793: F, t3851: F, t3879: F, t3882: F, t3888: F, t3889: F, t3902: F, t3905: F, t3909: F, t40000: F, t5210: F, t5215: F, t5230: F, t5250: F, t5321: F, t5334: F, t5335: F, t5344: F, t5349: F, t5351: F, t5353: F, t5354: F, t544: F, t54739: F, t54745: F, t54817: F, t54840: F, t54854: F, t54858: F, t54900: F, t54905: F, t54918: F, t54959: F, t54963: F, t54976: F, t55012: F, t553: F, t564: F, t568: F, t16448: F, t12020: F, t1842: F, t12023: F, t12026: F, t12030: F, t1372: F, t1385: F, t16022: F, t16122: F, t16436: F, t16439: F, t16475: F, t26224: F, t3887: F, t3911: F, t3912: F, t5326: F, t16468: F, t16458: F, t12237: F, t16437: F, t16460: F, t1834: F, t1843: F, t3752: F, t3758: F, t39910: F, t5318: F, t54738: F, t562: F, t12466: F, t12477: F, t1297: F, t1390: F, t193: F, t39249: F, t39256: F, t39261: F, t39266: F, t39304: F, t39309: F, t39312: F, t39316: F, t5126: F, t5161: F, t5308: F, t533: F, t53778: F, t53780: F, t53783: F, t53788: F, t53789: F, t53797: F, t53799: F, t53800: F, t53856: F, t54832: F, t16486: F, t3701: F, t1388: F, t3914: F, t15899: F, t16148: F, t16497: F, t3719: F, t3918: F, t3919: F, t39338: F, t39346: F, t39349: F, t39356: F, t5160: F, t54321: F, t54322: F, t54324: F, t1307: F, t3698: F, t3734: F, t39384: F, t39393: F, t39397: F, t39400: F, t39408: F, t39411: F, t54385: F, t54388: F, t54390: F, t12156: F, t12303: F, t16153: F, t16490: F, t1845: F, t39483: F, t5122: F, t5187: F, t54404: F, t54406: F, t54409: F, t54411: F, t54413: F, t39518: F, t39521: F, t39529: F, t39539: F, t54420: F, t54421: F, t54422: F, t54423: F, t54424: F, t54425: F, t54427: F, t12461: F, t5356: F, t19577: F, t22578: F, t39367: F, t39585: F, t39590: F, t39593: F, t39595: F, t54433: F, t54435: F, t54436: F, t40611: F, t12458: F, t15868: F, t15883: F, t15904: F, t16018: F, t39639: F, t5131: F, t54447: F, t54448: F, t54449: F, t54450: F, t54452: F, t571: F, t40224: F, t40230: F, t54465: F, t54466: F, t54468: F, t54470: F, t54472: F, t54473: F, t54475: F, t54476: F, t54478: F, t11972: F, t12012: F, t12451: F, t12550: F, t12725: F, t12734: F, t12823: F, t1459: F, t15857: F, t15872: F, t1774: F, t1799: F, t2314: F, t2323: F, t3652: F, t3660: F, t39235: F, t3929: F, t39320: F, t39324: F, t39327: F, t39350: F, t39360: F, t39364: F, t39373: F, t39463: F, t39468: F, t39472: F, t39476: F, t39490: F, t39496: F, t39499: F, t39502: F, t39505: F, t39508: F, t39549: F, t39563: F, t39570: F, t39577: F, t39615: F, t39655: F, t39658: F, t39844: F, t4028: F, t4034: F, t4072: F, t4073: F, t46117: F, t510: F, t5118: F, t5127: F, t513: F, t5361: F, t54313: F, t54315: F, t54317: F, t54318: F, t54319: F, t54320: F, t54326: F, t54376: F, t54379: F, t54381: F, t54383: F, t54384: F, t54393: F, t54396: F, t54399: F, t54401: F, t54403: F, t54414: F, t54419: F, t54429: F, t54430: F, t54431: F, t54437: F, t54438: F, t54439: F, t54442: F, t54443: F, t54444: F, t54445: F, t54446: F, t54453: F, t54455: F, t54457: F, t54459: F, t54461: F, t54463: F, t54464: F, t652: F, t671: F, t9416: F) -> F {
        let t55059 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2698::<F>(t1338, t16413, t12168, t12181, t12238, t1332, t1336, t1352, t1380, t1381, t16052, t16055, t16060, t16206, t16414, t1825, t1840, t3901, t3907, t40479, t5234, t5348, t53909, t54527);
        let t55088 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2699::<F>(t16463, t225, t12021, t12027, t12033, t12178, t12179, t12240, t12241, t12244, t12251, t12252, t12255, t12256, t12260, t12267, t12434, t12438, t12444, t1323, t1336, t1352, t1375, t1378, t1383, t1386, t16030, t16033, t16036, t16037, t16040, t16044, t16047, t16048, t16049, t16055, t16060, t16065, t16123, t16133, t16136, t16413, t16416, t16419, t16423, t16428, t16429, t16453, t16471, t1807, t19654, t3773, t3777, t3793, t3851, t3879, t3882, t3888, t3889, t3902, t3905, t3909, t40000, t5210, t5215, t5230, t5234, t5250, t5321, t5334, t5335, t5344, t5349, t5351, t5353, t5354, t544, t54739, t54745, t54817, t54840, t54854, t54858, t54900, t54905, t54918, t54959, t54963, t54976, t55012, t55059, t553, t564, t568);
        let t55124 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2700::<F>(t16448, t225, t12020, t1842, t12023, t12026, t12030, t1372, t1375, t1385, t1386, t16022, t16030, t16122, t16436, t16439, t16475, t26224, t3882, t3887, t3889, t3911, t3912, t5215, t5326, t5353, t5354, t568);
        let t55155 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2701::<F>(t16468, t225, t16458, t12023, t12027, t12033, t12237, t12444, t1386, t16022, t16437, t16453, t16460, t1834, t1843, t3752, t3758, t3882, t3889, t39910, t5318, t5321, t5326, t54738, t562, t568);
        let t55161 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2702::<F>(t12466, t12477, t1297, t1390, t193, t39249, t39256, t39261, t39266, t39304, t39309, t39312, t39316, t5126, t5161, t5308, t533, t53778, t53780, t53783, t53788, t53789, t53797, t53799, t53800, t53856, t54832, t55088, t55124, t55155);
        let t55180 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2703::<F>(t16486, t3701, t1388, t3914, t15899, t16148, t16497, t3719, t3918, t3919, t39338, t39346, t39349, t39356, t5126, t5160, t54321, t54322, t54324);
        let (t55183, t55195) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2704::<F>(t1307, t3698, t1390, t16486, t16497, t3734, t3918, t39384, t39393, t39397, t39400, t39408, t39411, t5126, t54385, t54388, t54390);
        let t55217 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2705::<F>(t12156, t12303, t12477, t1390, t16153, t16490, t1845, t193, t3918, t3919, t39483, t5122, t5126, t5187, t54404, t54406, t54409, t54411, t54413);
        let (t55224, t55228) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2706::<F>(t1307, t193, t39518, t39521, t39529, t39539, t54420, t54421, t54422, t54423, t54424, t54425, t54427);
        let t55256 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2707::<F>(t12461, t5356, t1388, t3719, t19577, t22578, t3698, t3918, t39367, t39585, t39590, t39593, t39595, t5160, t5161, t54433, t54435, t54436);
        let t55280 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2708::<F>(t193, t3734, t1845, t40611, t12458, t1307, t15868, t15883, t15904, t16018, t3719, t3918, t39639, t5126, t5131, t5160, t54447, t54448, t54449, t54450, t54452, t571);
        let t55315 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2709::<F>(t40224, t40230, t54465, t54466, t54468, t54470, t54472, t54473, t54475, t54476, t54478, t11972, t12012, t12451, t12466, t12550, t12725, t12734, t12823, t1459, t15857, t15868, t15872, t15899, t16018, t1774, t1799, t2314, t2323, t3652, t3660, t3914, t3918, t3919, t39235, t3929, t39320, t39324, t39327, t39350, t39360, t39364, t39373, t39463, t39468, t39472, t39476, t39490, t39496, t39499, t39502, t39505, t39508, t39549, t39563, t39570, t39577, t39615, t39655, t39658, t39844, t4028, t4034, t4072, t4073, t46117, t510, t5118, t5122, t5126, t5127, t513, t5160, t5161, t5187, t5361, t54313, t54315, t54317, t54318, t54319, t54320, t54326, t54376, t54379, t54381, t54383, t54384, t54393, t54396, t54399, t54401, t54403, t54414, t54419, t54429, t54430, t54431, t54437, t54438, t54439, t54442, t54443, t54444, t54445, t54446, t54453, t54455, t54457, t54459, t54461, t54463, t54464, t55161, t55180, t55183, t55195, t55217, t55224, t55228, t55256, t55280, t652, t671, t9416);
    t55315
}
