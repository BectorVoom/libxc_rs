//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta722 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2299;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2300;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2301;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2302;
use chunk4::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2303;
use chunk5::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2304;
use chunk6::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2305;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta722(t1184: f64, t6139: f64, t1716: f64, t1752: f64, t17686: f64, t2155: f64, t24589: f64, t24590: f64, t24601: f64, t24633: f64, t24638: f64, t254: f64, t27406: f64, t27412: f64, t27549: f64, t27747: f64, t27774: f64, t27775: f64, t27786: f64, t27799: f64, t29816: f64, t4945: f64, t6140: f64, t66860: f64, t7283: f64, t94349: f64, t94458: f64, t94503: f64, t94584: f64, t94676: f64, t225: f64, t29685: f64, t103218: f64, t1238: f64, t1252: f64, t19208: f64, t19232: f64, t19234: f64, t2154: f64, t27752: f64, t27794: f64, t27812: f64, t29798: f64, t29812: f64, t3593: f64, t3598: f64, t5055: f64, t5088: f64, t7291: f64, t7356: f64, t7392: f64, t8087: f64, t94700: f64, t94701: f64, t103345: f64, t2122: f64, t24574: f64, t29674: f64, t29750: f64, t85853: f64, t1011: f64, t6218: f64, t29624: f64, t29614: f64, t103223: f64, t19189: f64, t24788: f64, t24812: f64, t24833: f64, t27461: f64, t27473: f64, t27489: f64, t27516: f64, t27553: f64, t29740: f64, t29744: f64, t4978: f64, t7364: f64, t7373: f64, t7375: f64, t7376: f64, t94784: f64, t94787: f64, t24826: f64, t29782: f64, t29736: f64, t86094: f64, t17635: f64, t17691: f64, t24849: f64, t24851: f64, t27507: f64, t27521: f64, t27526: f64, t27550: f64, t27551: f64, t27558: f64, t27561: f64, t27563: f64, t29758: f64, t29762: f64, t72164: f64, t94395: f64, t94920: f64, t95092: f64, t131: f64, t467: f64, t5415: f64, t6794: f64, t29734: f64, t607: f64, t29754: f64, t3032: f64, t6224: f64, t18301: f64, t19173: f64, t27638: f64, t29749: f64, t29776: f64, t7378: f64, t8066: f64, t85859: f64, t85963: f64, t86015: f64, t86037: f64, t86076: f64, t86077: f64, t94948: f64, t95000: f64, t95005: f64, t95035: f64, t1209: f64, t1751: f64, t24813: f64, t27490: f64, t27491: f64, t27496: f64, t27497: f64, t27501: f64, t27536: f64, t27644: f64, t3247: f64, t3502: f64, t3961: f64, t5012: f64, t94796: f64, t94797: f64, t94847: f64, t94881: f64, t94885: f64, t94889: f64, t94891: f64, t94901: f64, t94954: f64, t94963: f64, t2147: f64, t8034: f64, t7327: f64, t1653: f64, t18241: f64, t19128: f64, t24858: f64, t27454: f64, t27462: f64, t27552: f64, t29720: f64, t3604: f64, t5979: f64, t7362: f64, t7363: f64, t7377: f64, t94911: f64, t94941: f64, t94947: f64, t95794: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t103422, t103457) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2299(t1184, t6139, t1716, t1752, t17686, t2155, t24589, t24590, t24601, t24633, t24638, t254, t27406, t27412, t27549, t27747, t27774, t27775, t27786, t27799, t29816, t4945, t6140, t66860, t7283, t94349, t94458, t94503, t94584, t94676);
        let t103488 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2300(t225, t29685, t103218, t1238, t1252, t19208, t19232, t19234, t2154, t24633, t27406, t27747, t27752, t27794, t27812, t29798, t29812, t3593, t3598, t5055, t5088, t7283, t7291, t7356, t7392, t8087, t94700, t94701);
        let (t103490, t103494, t103515, t103538) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2301(t103345, t2122, t24574, t29674, t29750, t85853, t1011, t6218, t225, t29624, t29614, t103223, t19189, t24589, t24788, t24812, t24833, t27461, t27473, t27489, t27516, t27553, t29740, t29744, t4978, t7364, t7373, t7375, t7376, t94784, t94787);
        let t103577 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2302(t24826, t29782, t29736, t86094, t17635, t17686, t17691, t24589, t24788, t24849, t24851, t27507, t27521, t27526, t27549, t27550, t27551, t27558, t27561, t27563, t29758, t29762, t72164, t7376, t94395, t94920, t95092);
        let (t103615, t103624) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2303(t131, t467, t5415, t6794, t29734, t607, t7376, t29754, t85853, t3032, t6224, t17691, t18301, t19173, t24589, t24788, t24812, t24849, t27549, t27550, t27551, t27638, t29749, t29776, t4978, t7373, t7375, t7378, t8066, t85859, t85963, t86015, t86037, t86076, t86077, t94948, t95000, t95005, t95035);
        let t103659 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2304(t1209, t1751, t17686, t24589, t24812, t24813, t27490, t27491, t27496, t27497, t27501, t27536, t27550, t27644, t29734, t3247, t3502, t3961, t5012, t7373, t86037, t94796, t94797, t94847, t94881, t94885, t94889, t94891, t94901, t94954, t94963);
        let (t103683, t103693) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2305(t2147, t8034, t29624, t7327, t103422, t1653, t18241, t19128, t24858, t27406, t27454, t27462, t27549, t27552, t29720, t3604, t5979, t7283, t7362, t7363, t7373, t7375, t7376, t7377, t94911, t94941, t94947, t95794);
    (t103457, t103488, t103490, t103494, t103515, t103538, t103577, t103615, t103624, t103659, t103683, t103693)
}
