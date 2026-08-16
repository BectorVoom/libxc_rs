//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta725 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2329;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2330;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2331;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2332;
use chunk4::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2333;
use chunk5::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2334;
use chunk6::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2335;
use chunk7::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2336;
use chunk8::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2337;
use chunk9::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2338;
use chunk10::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2339;
use chunk11::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2340;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta725(t1210: f64, t24721: f64, t29593: f64, t27700: f64, t95422: f64, t2132: f64, t2136: f64, t5398: f64, t19040: f64, t7345: f64, t18392: f64, t27617: f64, t4993: f64, t18215: f64, t2121: f64, t2133: f64, t24736: f64, t27703: f64, t4899: f64, t6138: f64, t6203: f64, t7321: f64, t8027: f64, t95540: f64, t95542: f64, t95545: f64, t28525: f64, t461: f64, t7324: f64, t18342: f64, t18346: f64, t18590: f64, t18594: f64, t18997: f64, t19068: f64, t27604: f64, t27674: f64, t4974: f64, t4989: f64, t5046: f64, t7310: f64, t7331: f64, t95550: f64, t95571: f64, t95573: f64, t210: f64, t29584: f64, t27683: f64, t27710: f64, t1198: f64, t27684: f64, t27692: f64, t27711: f64, t6192: f64, t8040: f64, t86330: f64, t86348: f64, t86350: f64, t95323: f64, t95556: f64, t95587: f64, t95590: f64, t95593: f64, t95617: f64, t95588: f64, t18975: f64, t18332: f64, t1222: f64, t29606: f64, t1748: f64, t18584: f64, t24741: f64, t27580: f64, t27655: f64, t27687: f64, t27714: f64, t5030: f64, t6232: f64, t7999: f64, t8031: f64, t8035: f64, t86167: f64, t95452: f64, t95662: f64, t95702: f64, t104029: f64, t104056: f64, t104087: f64, t104101: f64, t104134: f64, t104155: f64, t104193: f64, t104220: f64, t104264: f64, t104292: f64, t104319: f64, t104351: f64, t29787: f64, t85639: f64, t1170: f64, t29726: f64, t103337: f64, t1244: f64, t1246: f64, t15027: f64, t1716: f64, t19201: f64, t2147: f64, t27454: f64, t27471: f64, t27507: f64, t27511: f64, t27543: f64, t27725: f64, t470: f64, t491: f64, t4928: f64, t493: f64, t5064: f64, t6218: f64, t7283: f64, t7348: f64, t7387: f64, t95768: f64, t95774: f64, t24574: f64, t29557: f64, t29551: f64, t8003: f64, t94490: f64, t103218: f64, t103490: f64, t103494: f64, t103538: f64, t103577: f64, t103624: f64, t103659: f64, t103693: f64, t103733: f64, t103766: f64, t103801: f64, t103829: f64, t103864: f64, t103889: f64, t103918: f64, t103949: f64, t103978: f64, t104002: f64, t1186: f64, t1238: f64, t1241: f64, t14980: f64, t2122: f64, t24567: f64, t24638: f64, t27411: f64, t27751: f64, t29545: f64, t29670: f64, t497: f64, t6146: f64, t7303: f64, t8088: f64, t94710: f64, t29694: f64, t29678: f64, t7280: f64, t14972: f64, t15820: f64, t1761: f64, t18571: f64, t2144: f64, t24893: f64, t27383: f64, t27396: f64, t27406: f64, t27427: f64, t29795: f64, t3487: f64, t4945: f64, t498: f64, t6150: f64, t6268: f64, t86451: f64, t94759: f64, t95899: f64, t225: f64, t29687: f64, t1252: f64, t1721: f64, t2155: f64, t254: f64, t27549: f64, t27742: f64, t27761: f64, t27767: f64, t27775: f64, t27779: f64, t27786: f64, t29532: f64, t3593: f64, t466: f64, t5055: f64, t65208: f64, t94514: f64, t94779: f64, t95824: f64, t95902: f64, t27817: f64, t15797: f64, t1751: f64, t17635: f64, t19209: f64, t19220: f64, t19232: f64, t24589: f64, t24601: f64, t27444: f64, t27453: f64, t27766: f64, t3598: f64, t460: f64, t4940: f64, t6267: f64, t7286: f64, t7351: f64, t7391: f64, t7392: f64, t8054: f64, t8061: f64, t86473: f64, t95834: f64, t29813: f64, t1760: f64, t19213: f64, t19219: f64, t19225: f64, t24602: f64, t24615: f64, t27389: f64, t27741: f64, t27784: f64, t27785: f64, t27830: f64, t3966: f64, t5060: f64, t7300: f64, t8002: f64, t94358: f64, t94680: f64, t95863: f64, t95866: f64, t95884: f64, t95889: f64) -> (f64, f64, f64, f64, f64) {
        let (t104355, t104364, t104367, t104369, t104371, t104375) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2329(t1210, t24721, t29593, t27700, t95422, t2132, t2136, t5398, t19040, t7345, t18392, t27617, t4993);
        let t104380 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2330(t104355, t104364, t104367, t104369, t104371, t104375, t18215, t2121, t2132, t2133, t24736, t27703, t4899, t6138, t6203, t7321, t8027, t95540, t95542, t95545);
        let t104404 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2331(t28525, t461, t7324, t18342, t18346, t18590, t18594, t18997, t19068, t27604, t27617, t27674, t4974, t4989, t5046, t7310, t7331, t7345, t95550, t95571, t95573);
        let t104424 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2332(t210, t29584, t27683, t27710, t1198, t27684, t27692, t27711, t6192, t7331, t8040, t86330, t86348, t86350, t95323, t95556, t95587, t95590, t95593, t95617);
        let t104449 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2333(t27700, t95588, t18975, t7345, t18332, t7310, t1222, t29606, t1748, t18584, t24741, t27580, t27604, t27655, t27687, t27714, t5030, t6232, t7999, t8031, t8035, t86167, t95452, t95662, t95702);
        let t104453 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2334(t104029, t104056, t104087, t104101, t104134, t104155, t104193, t104220, t104264, t104292, t104319, t104351, t104380, t104404, t104424, t104449);
        let t104482 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2335(t29787, t85639, t1170, t2121, t29726, t103337, t104453, t1244, t1246, t15027, t1716, t19201, t2147, t27454, t27471, t27507, t27511, t27543, t27725, t470, t491, t4928, t493, t5064, t6218, t7283, t7348, t7387, t95768, t95774);
        let t104508 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2336(t24574, t29557, t29551, t8003, t94490, t103218, t103490, t103494, t103538, t103577, t103624, t103659, t103693, t103733, t103766, t103801, t103829, t103864, t103889, t103918, t103949, t103978, t104002, t104482, t1186, t1238, t1241, t14980, t1716, t2122, t24567, t24638, t27411, t27751, t29545, t29670, t4928, t497, t6146, t7283, t7303, t8088, t94710);
        let t104534 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2337(t24574, t29694, t1170, t2121, t29670, t29678, t7280, t14972, t15820, t1761, t18571, t2144, t24893, t27383, t27396, t27406, t27427, t29795, t3487, t4945, t498, t6150, t6268, t7348, t8088, t86451, t94759, t95899);
        let t104564 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2338(t225, t29687, t104453, t1252, t1721, t1761, t2155, t254, t27396, t27406, t27549, t27742, t27761, t27767, t27775, t27779, t27786, t29532, t3593, t466, t498, t5055, t65208, t7999, t94514, t94779, t95824, t95902);
        let t104596 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2339(t27817, t7999, t1238, t14972, t15797, t1716, t1751, t17635, t19209, t19220, t19232, t24589, t24601, t27444, t27453, t27766, t29795, t3593, t3598, t460, t4940, t498, t6267, t7283, t7286, t7351, t7391, t7392, t8054, t8061, t86473, t95834);
        let t104631 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2340(t24574, t29813, t1238, t14980, t1760, t19213, t19219, t19225, t24589, t24601, t24602, t24615, t27389, t27406, t27741, t27784, t27785, t27830, t3598, t3966, t5060, t7283, t7300, t8002, t8061, t94358, t94680, t95863, t95866, t95884, t95889);
    (t104508, t104534, t104564, t104596, t104631)
}
