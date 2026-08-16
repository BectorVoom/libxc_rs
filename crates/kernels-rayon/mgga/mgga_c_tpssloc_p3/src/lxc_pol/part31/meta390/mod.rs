//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta390 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1394;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1395;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1396;
use chunk3::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1397;
use chunk4::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1398;
use chunk5::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1399;
use chunk6::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1400;
use chunk7::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1401;
use chunk8::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1402;
use chunk9::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1403;
use chunk10::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1404;
use chunk11::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1405;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta390(t10165: f64, t18070: f64, t225: f64, t5915: f64, t1049: f64, t5872: f64, t3201: f64, t3188: f64, t1057: f64, t18028: f64, t1615: f64, t4657: f64, t1060: f64, t1022: f64, t360: f64, t6739: f64, t5928: f64, t5866: f64, t11066: f64, t4649: f64, t1629: f64, t11060: f64, t4684: f64, t5936: f64, t4673: f64, t1058: f64, t1061: f64, t11034: f64, t11037: f64, t11046: f64, t11059: f64, t11065: f64, t14618: f64, t14651: f64, t1630: f64, t3180: f64, t3186: f64, t3200: f64, t4674: f64, t5929: f64, t5937: f64, t5939: f64, t18047: f64, t383: f64, t5932: f64, t1625: f64, t5914: f64, t17959: f64, t381: f64, t1003: f64, t1063: f64, t14608: f64, t1610: f64, t1632: f64, t17876: f64, t353: f64, t384: f64, t4615: f64, t4669: f64, t4678: f64, t4681: f64, t4685: f64, t4689: f64, t4691: f64, t5903: f64, t5933: f64, t5941: f64, t1055: f64, t1052: f64, t1066: f64, t14529: f64, t14545: f64, t14552: f64, t14555: f64, t1635: f64, t18053: f64, t18057: f64, t18059: f64, t18062: f64, t18065: f64, t388: f64, t4660: f64, t4665: f64, t18050: f64, t1068: f64, t1070: f64, t17194: f64, t17197: f64, t17198: f64, t17202: f64, t17209: f64, t17301: f64, t17303: f64, t17306: f64, t17372: f64, t17374: f64, t17377: f64, t17379: f64, t17425: f64, t17427: f64, t17561: f64, t17563: f64, t17568: f64, t193: f64, t336: f64, t4696: f64, t4700: f64, t4701: f64, t17490: f64, t17504: f64, t17506: f64, t17509: f64, t17512: f64, t17515: f64, t17519: f64, t17523: f64, t17526: f64, t17530: f64, t17929: f64, t17932: f64, t17936: f64, t17940: f64, t17942: f64, t17944: f64, t17946: f64, t17950: f64, t17953: f64, t17957: f64, t25: f64, t265: f64, t394: f64, t17133: f64, t1074: f64, t1408: f64, t1409: f64, t1642: f64, t16557: f64, t16558: f64, t17141: f64, t396: f64, t3966: f64, t40: f64, t4324: f64, t4705: f64, t5397: f64, t5398: f64, t5669: f64, t5955: f64, t606: f64, t607: f64, t873: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64, t5972: f64, t690: f64, t11147: f64, t5392: f64, t11145: f64, t123: f64, t11153: f64, t3240: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t18071, t18074, t18081, t18083, t18086, t18088) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1394(t10165, t18070, t225, t5915, t1049, t5872, t3201, t3188, t1057, t18028, t1615, t4657);
        let (t18089, t18094, t18100, t18104, t18108) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1395(t1060, t18088, t1022, t360, t6739, t5928, t1049, t5866, t11066, t3201, t4649, t1629);
        let t18124 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1396(t1022, t11060, t5928, t4684, t5936, t4673, t1058, t1061, t11034, t11037, t11046, t11059, t11065, t14618, t14651, t1630, t18081, t18083, t18086, t18089, t18094, t18100, t18104, t18108, t3180, t3186, t3200, t4674, t5929, t5937, t5939);
        let (t18129, t18131, t18139, t18142, t18151, t18154) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1397(t18047, t383, t4684, t5932, t3188, t4649, t1629, t4673, t1625, t1060, t1022, t5914);
        let t18164 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1398(t1060, t18154, t17959, t381, t1003, t1058, t1063, t14608, t1610, t1632, t17876, t18129, t18131, t18139, t18142, t18151, t3180, t3186, t3200, t353, t384, t4615, t4669, t4678, t4681, t4685, t4689, t4691, t5903, t5933, t5941);
        let t18168 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1399(t18124, t18164, t1055, t1052, t1066, t14529, t14545, t14552, t14555, t1635, t18053, t18057, t18059, t18062, t18065, t18071, t18074, t388, t4660, t4665);
        let t18173 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1400(t18050, t18168, t1068, t1070, t17194, t17197, t17198, t17202, t17209, t17301, t17303, t17306, t17372, t17374, t17377, t17379, t17425, t17427, t17561, t17563, t17568, t193, t336, t4696, t4700, t4701);
        let t18174 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1401(t17490, t17504, t17506, t17509, t17512, t17515, t17519, t17523, t17526, t17530, t17929, t17932, t17936, t17940, t17942, t17944, t17946, t17950, t17953, t17957);
        let t18188 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1402(t25, t265, t394, t17133, t18173, t18174, t1074, t1408, t1409, t1642, t16557, t16558, t17141, t396, t3966, t40, t4324, t4705, t5397, t5398, t5669, t5955, t606, t607, t873, dens_threshold, rho0, zeta_threshold);
        let (t18196, t18203) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1403(t16557, t5972, t690);
        let (t18206, t18208) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1404(t11147, t5392, t607, t11145, t123);
        let (t18211, t18213) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1405(t11153, t5392, t607, t3240, t123);
    (t18188, t18196, t18203, t18206, t18208, t18211, t18213)
}
