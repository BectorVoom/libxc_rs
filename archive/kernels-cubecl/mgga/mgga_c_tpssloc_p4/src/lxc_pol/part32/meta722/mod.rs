//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta722 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2299;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2300;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2301;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2302;
use chunk4::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2303;
use chunk5::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2304;
use chunk6::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2305;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta722<F: Float>(t1184: F, t6139: F, t1716: F, t1752: F, t17686: F, t2155: F, t24589: F, t24590: F, t24601: F, t24633: F, t24638: F, t254: F, t27406: F, t27412: F, t27549: F, t27747: F, t27774: F, t27775: F, t27786: F, t27799: F, t29816: F, t4945: F, t6140: F, t66860: F, t7283: F, t94349: F, t94458: F, t94503: F, t94584: F, t94676: F, t225: F, t29685: F, t103218: F, t1238: F, t1252: F, t19208: F, t19232: F, t19234: F, t2154: F, t27752: F, t27794: F, t27812: F, t29798: F, t29812: F, t3593: F, t3598: F, t5055: F, t5088: F, t7291: F, t7356: F, t7392: F, t8087: F, t94700: F, t94701: F, t103345: F, t2122: F, t24574: F, t29674: F, t29750: F, t85853: F, t1011: F, t6218: F, t29624: F, t29614: F, t103223: F, t19189: F, t24788: F, t24812: F, t24833: F, t27461: F, t27473: F, t27489: F, t27516: F, t27553: F, t29740: F, t29744: F, t4978: F, t7364: F, t7373: F, t7375: F, t7376: F, t94784: F, t94787: F, t24826: F, t29782: F, t29736: F, t86094: F, t17635: F, t17691: F, t24849: F, t24851: F, t27507: F, t27521: F, t27526: F, t27550: F, t27551: F, t27558: F, t27561: F, t27563: F, t29758: F, t29762: F, t72164: F, t94395: F, t94920: F, t95092: F, t131: F, t467: F, t5415: F, t6794: F, t29734: F, t607: F, t29754: F, t3032: F, t6224: F, t18301: F, t19173: F, t27638: F, t29749: F, t29776: F, t7378: F, t8066: F, t85859: F, t85963: F, t86015: F, t86037: F, t86076: F, t86077: F, t94948: F, t95000: F, t95005: F, t95035: F, t1209: F, t1751: F, t24813: F, t27490: F, t27491: F, t27496: F, t27497: F, t27501: F, t27536: F, t27644: F, t3247: F, t3502: F, t3961: F, t5012: F, t94796: F, t94797: F, t94847: F, t94881: F, t94885: F, t94889: F, t94891: F, t94901: F, t94954: F, t94963: F, t2147: F, t8034: F, t7327: F, t1653: F, t18241: F, t19128: F, t24858: F, t27454: F, t27462: F, t27552: F, t29720: F, t3604: F, t5979: F, t7362: F, t7363: F, t7377: F, t94911: F, t94941: F, t94947: F, t95794: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t103422, t103457) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2299::<F>(t1184, t6139, t1716, t1752, t17686, t2155, t24589, t24590, t24601, t24633, t24638, t254, t27406, t27412, t27549, t27747, t27774, t27775, t27786, t27799, t29816, t4945, t6140, t66860, t7283, t94349, t94458, t94503, t94584, t94676);
        let t103488 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2300::<F>(t225, t29685, t103218, t1238, t1252, t19208, t19232, t19234, t2154, t24633, t27406, t27747, t27752, t27794, t27812, t29798, t29812, t3593, t3598, t5055, t5088, t7283, t7291, t7356, t7392, t8087, t94700, t94701);
        let (t103490, t103494, t103515, t103538) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2301::<F>(t103345, t2122, t24574, t29674, t29750, t85853, t1011, t6218, t225, t29624, t29614, t103223, t19189, t24589, t24788, t24812, t24833, t27461, t27473, t27489, t27516, t27553, t29740, t29744, t4978, t7364, t7373, t7375, t7376, t94784, t94787);
        let t103577 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2302::<F>(t24826, t29782, t29736, t86094, t17635, t17686, t17691, t24589, t24788, t24849, t24851, t27507, t27521, t27526, t27549, t27550, t27551, t27558, t27561, t27563, t29758, t29762, t72164, t7376, t94395, t94920, t95092);
        let (t103615, t103624) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2303::<F>(t131, t467, t5415, t6794, t29734, t607, t7376, t29754, t85853, t3032, t6224, t17691, t18301, t19173, t24589, t24788, t24812, t24849, t27549, t27550, t27551, t27638, t29749, t29776, t4978, t7373, t7375, t7378, t8066, t85859, t85963, t86015, t86037, t86076, t86077, t94948, t95000, t95005, t95035);
        let t103659 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2304::<F>(t1209, t1751, t17686, t24589, t24812, t24813, t27490, t27491, t27496, t27497, t27501, t27536, t27550, t27644, t29734, t3247, t3502, t3961, t5012, t7373, t86037, t94796, t94797, t94847, t94881, t94885, t94889, t94891, t94901, t94954, t94963);
        let (t103683, t103693) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2305::<F>(t2147, t8034, t29624, t7327, t103422, t1653, t18241, t19128, t24858, t27406, t27454, t27462, t27549, t27552, t29720, t3604, t5979, t7283, t7362, t7363, t7373, t7375, t7376, t7377, t94911, t94941, t94947, t95794);
    (t103457, t103488, t103490, t103494, t103515, t103538, t103577, t103615, t103624, t103659, t103683, t103693)
}
