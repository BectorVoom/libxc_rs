//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta684 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2326;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2327;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2328;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2329;
use chunk4::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2330;
use chunk5::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2331;
use chunk6::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2332;
use chunk7::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2333;
use chunk8::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2334;
use chunk9::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2335;
use chunk10::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2336;
use chunk11::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2337;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta684<F: Float>(t24682: F, t460: F, t95484: F, t27634: F, t3030: F, t86259: F, t24740: F, t5064: F, t15640: F, t24729: F, t14726: F, t15394: F, t2121: F, t2132: F, t2133: F, t24706: F, t27639: F, t27645: F, t27674: F, t27704: F, t3552: F, t3557: F, t3580: F, t4928: F, t7321: F, t7331: F, t86365: F, t86368: F, t95260: F, t95285: F, t95316: F, t95343: F, t95367: F, t95407: F, t95443: F, t95469: F, t95492: F, t95518: F, t95543: F, t95576: F, t95603: F, t95633: F, t95672: F, t24574: F, t27574: F, t24844: F, t7999: F, t1244: F, t1246: F, t15015: F, t15027: F, t1729: F, t24792: F, t24863: F, t27470: F, t27724: F, t3471: F, t3493: F, t3624: F, t470: F, t493: F, t5079: F, t7283: F, t7373: F, t7375: F, t7376: F, t8054: F, t8077: F, t86020: F, t3427: F, t27517: F, t85639: F, t27481: F, t11888: F, t11904: F, t15022: F, t15247: F, t24589: F, t24794: F, t24798: F, t24841: F, t24849: F, t27516: F, t27532: F, t27543: F, t3565: F, t5072: F, t7327: F, t8082: F, t8085: F, t86057: F, t7365: F, t94490: F, t1715: F, t974: F, t24847: F, t24771: F, t15418: F, t2127: F, t221: F, t27553: F, t11877: F, t11907: F, t11914: F, t15245: F, t15429: F, t24765: F, t24834: F, t24838: F, t27406: F, t27454: F, t27546: F, t8083: F, t86073: F, t86095: F, t94588: F, t477: F, t5052: F, t27654: F, t24745: F, t4935: F, t1090: F, t1186: F, t1201: F, t1215: F, t15771: F, t2147: F, t24799: F, t24851: F, t27525: F, t27549: F, t27552: F, t27722: F, t27732: F, t3966: F, t462: F, t7362: F, t7364: F, t7377: F, t86106: F, t86113: F, t86116: F, t94976: F, t24585: F, t27800: F, t225: F, t27805: F, t11613: F, t1191: F, t1238: F, t1241: F, t1252: F, t15802: F, t1720: F, t2155: F, t24612: F, t24757: F, t24897: F, t254: F, t27784: F, t27785: F, t27786: F, t27792: F, t3631: F, t4940: F, t498: F, t5055: F, t53703: F, t7348: F, t8088: F, t94779: F, t94820: F, t94867: F, t94902: F, t94942: F, t94980: F, t95026: F, t95058: F, t95087: F, t95122: F, t95150: F, t95184: F, t95224: F, t27392: F, t1170: F, t27766: F, t15794: F, t1716: F, t24567: F, t24568: F, t24582: F, t24630: F, t24639: F, t24877: F, t24893: F, t27415: F, t3598: F, t3630: F, t4945: F, t5060: F, t7351: F, t8087: F, t86473: F, t86494: F, t2154: F, t45349: F, t27776: F, t11147: F, t497: F, t27424: F, t27422: F, t27752: F, t14165: F, t15787: F, t15793: F, t24601: F, t24888: F, t27830: F, t466: F, t7300: F, t8002: F, t8010: F, t85674: F, t85750: F, t86501: F, t94796: F, t27834: F, t3640: F, t11947: F, t8090: F, t1254: F, t1256: F, t15834: F, t1763: F, t193: F, t24905: F, t24909: F, t27838: F, t27843: F, t336: F, t3633: F, t3637: F, t4700: F, t5091: F, t64447: F, t7398: F, t86513: F, t86517: F, t86524: F, t94341: F, t94385: F, t94428: F, t94464: F, t94498: F, t94530: F, t94564: F, t94605: F, t94637: F, t94673: F, t94698: F, t94734: F, t94770: F, t28: F, t265: F, t504: F, t89823: F, t12606: F, t1409: F, t2161: F, t2250: F, t24916: F, t27850: F, t52: F, t607: F, t7402: F, t8097: F, t90003: F, dens_threshold: F, rho1: F, zeta_threshold: F, t113: F, t12545: F, t12835: F, t1393: F, t24932: F, t27903: F, t4077: F, t7266: F, t91602: F, t91606: F, t91608: F, t91610: F, t91612: F, t91623: F, t91625: F, t91627: F, t91630: F, t91637: F, t91640: F, t91642: F, t91657: F, t91662: F, t94293: F) -> F {
        let t95703 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2326::<F>(t24682, t460, t95484, t27634, t3030, t86259, t24740, t5064, t15640, t24729, t14726, t15394, t2121, t2132, t2133, t24706, t27639, t27645, t27674, t27704, t3552, t3557, t3580, t4928, t7321, t7331, t86365, t86368);
        let t95707 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2327::<F>(t95260, t95285, t95316, t95343, t95367, t95407, t95443, t95469, t95492, t95518, t95543, t95576, t95603, t95633, t95672, t95703);
        let t95723 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2328::<F>(t24574, t27574, t24844, t7999, t1244, t1246, t15015, t15027, t1729, t24792, t24863, t27470, t27724, t3471, t3493, t3624, t470, t493, t5079, t7283, t7373, t7375, t7376, t8054, t8077, t86020, t95707);
        let t95752 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2329::<F>(t2121, t3427, t8077, t27517, t85639, t24574, t27481, t11888, t11904, t15022, t15247, t24589, t24794, t24798, t24841, t24849, t27516, t27532, t27543, t3565, t3624, t5064, t5072, t7327, t8082, t8085, t86057);
        let (t95772, t95779) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2330::<F>(t7365, t94490, t1715, t974, t24847, t24771, t7999, t15418, t2127, t221, t27553, t11877, t11907, t11914, t15245, t15429, t24765, t24834, t24838, t27406, t27454, t27546, t7283, t8082, t8083, t86073, t86095, t94588);
        let t95817 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2331::<F>(t477, t5052, t27654, t7327, t24745, t4935, t1090, t1186, t1201, t1215, t15771, t2121, t2147, t24589, t24799, t24849, t24851, t27406, t27525, t27549, t27552, t27722, t27732, t3966, t462, t7283, t7362, t7364, t7373, t7376, t7377, t86106, t86113, t86116, t94976);
        let t95844 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2332::<F>(t24585, t7999, t24574, t27800, t225, t27805, t11613, t1191, t1238, t1241, t1252, t15802, t1720, t2155, t24612, t24757, t24897, t254, t27784, t27785, t27786, t27792, t3631, t4940, t498, t5055, t53703, t7348, t8088, t94779, t94820, t94867, t94902, t94942, t94980, t95026, t95058, t95087, t95122, t95150, t95184, t95224, t95723, t95752, t95779, t95817);
        let t95876 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2333::<F>(t24574, t27392, t1170, t2121, t27766, t1238, t15794, t1716, t24567, t24568, t24582, t24630, t24639, t24877, t24893, t27406, t27415, t3598, t3630, t4945, t5055, t5060, t7283, t7351, t8087, t86473, t86494);
        let t95913 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2334::<F>(t2154, t45349, t27776, t95772, t11147, t497, t225, t27424, t27422, t24574, t27752, t1252, t14165, t15787, t15793, t24601, t24888, t27406, t27784, t27830, t3471, t3631, t466, t498, t7283, t7300, t7351, t8002, t8010, t85674, t85750, t86501, t94796, t95707);
        let t95952 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2335::<F>(t27834, t3640, t11947, t8090, t1254, t1256, t15834, t1763, t193, t24905, t24909, t27838, t27843, t336, t3633, t3637, t4700, t5091, t64447, t7398, t86513, t86517, t86524, t94341, t94385, t94428, t94464, t94498, t94530, t94564, t94605, t94637, t94673, t94698, t94734, t94770, t95844, t95876, t95913);
        let t95965 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2336::<F>(t28, t265, t504, t89823, t95952, t12606, t1409, t2161, t2250, t24916, t27850, t3966, t52, t607, t7402, t8097, t90003, dens_threshold, rho1, zeta_threshold);
        let t95970 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2337::<F>(t113, t12545, t12835, t1393, t24932, t27903, t4077, t7266, t91602, t91606, t91608, t91610, t91612, t91623, t91625, t91627, t91630, t91637, t91640, t91642, t91657, t91662, t94293, t95965);
    t95970
}
