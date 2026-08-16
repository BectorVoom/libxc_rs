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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

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
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta684(t24682: f64, t460: f64, t95484: f64, t27634: f64, t3030: f64, t86259: f64, t24740: f64, t5064: f64, t15640: f64, t24729: f64, t14726: f64, t15394: f64, t2121: f64, t2132: f64, t2133: f64, t24706: f64, t27639: f64, t27645: f64, t27674: f64, t27704: f64, t3552: f64, t3557: f64, t3580: f64, t4928: f64, t7321: f64, t7331: f64, t86365: f64, t86368: f64, t95260: f64, t95285: f64, t95316: f64, t95343: f64, t95367: f64, t95407: f64, t95443: f64, t95469: f64, t95492: f64, t95518: f64, t95543: f64, t95576: f64, t95603: f64, t95633: f64, t95672: f64, t24574: f64, t27574: f64, t24844: f64, t7999: f64, t1244: f64, t1246: f64, t15015: f64, t15027: f64, t1729: f64, t24792: f64, t24863: f64, t27470: f64, t27724: f64, t3471: f64, t3493: f64, t3624: f64, t470: f64, t493: f64, t5079: f64, t7283: f64, t7373: f64, t7375: f64, t7376: f64, t8054: f64, t8077: f64, t86020: f64, t3427: f64, t27517: f64, t85639: f64, t27481: f64, t11888: f64, t11904: f64, t15022: f64, t15247: f64, t24589: f64, t24794: f64, t24798: f64, t24841: f64, t24849: f64, t27516: f64, t27532: f64, t27543: f64, t3565: f64, t5072: f64, t7327: f64, t8082: f64, t8085: f64, t86057: f64, t7365: f64, t94490: f64, t1715: f64, t974: f64, t24847: f64, t24771: f64, t15418: f64, t2127: f64, t221: f64, t27553: f64, t11877: f64, t11907: f64, t11914: f64, t15245: f64, t15429: f64, t24765: f64, t24834: f64, t24838: f64, t27406: f64, t27454: f64, t27546: f64, t8083: f64, t86073: f64, t86095: f64, t94588: f64, t477: f64, t5052: f64, t27654: f64, t24745: f64, t4935: f64, t1090: f64, t1186: f64, t1201: f64, t1215: f64, t15771: f64, t2147: f64, t24799: f64, t24851: f64, t27525: f64, t27549: f64, t27552: f64, t27722: f64, t27732: f64, t3966: f64, t462: f64, t7362: f64, t7364: f64, t7377: f64, t86106: f64, t86113: f64, t86116: f64, t94976: f64, t24585: f64, t27800: f64, t225: f64, t27805: f64, t11613: f64, t1191: f64, t1238: f64, t1241: f64, t1252: f64, t15802: f64, t1720: f64, t2155: f64, t24612: f64, t24757: f64, t24897: f64, t254: f64, t27784: f64, t27785: f64, t27786: f64, t27792: f64, t3631: f64, t4940: f64, t498: f64, t5055: f64, t53703: f64, t7348: f64, t8088: f64, t94779: f64, t94820: f64, t94867: f64, t94902: f64, t94942: f64, t94980: f64, t95026: f64, t95058: f64, t95087: f64, t95122: f64, t95150: f64, t95184: f64, t95224: f64, t27392: f64, t1170: f64, t27766: f64, t15794: f64, t1716: f64, t24567: f64, t24568: f64, t24582: f64, t24630: f64, t24639: f64, t24877: f64, t24893: f64, t27415: f64, t3598: f64, t3630: f64, t4945: f64, t5060: f64, t7351: f64, t8087: f64, t86473: f64, t86494: f64, t2154: f64, t45349: f64, t27776: f64, t11147: f64, t497: f64, t27424: f64, t27422: f64, t27752: f64, t14165: f64, t15787: f64, t15793: f64, t24601: f64, t24888: f64, t27830: f64, t466: f64, t7300: f64, t8002: f64, t8010: f64, t85674: f64, t85750: f64, t86501: f64, t94796: f64, t27834: f64, t3640: f64, t11947: f64, t8090: f64, t1254: f64, t1256: f64, t15834: f64, t1763: f64, t193: f64, t24905: f64, t24909: f64, t27838: f64, t27843: f64, t336: f64, t3633: f64, t3637: f64, t4700: f64, t5091: f64, t64447: f64, t7398: f64, t86513: f64, t86517: f64, t86524: f64, t94341: f64, t94385: f64, t94428: f64, t94464: f64, t94498: f64, t94530: f64, t94564: f64, t94605: f64, t94637: f64, t94673: f64, t94698: f64, t94734: f64, t94770: f64, t28: f64, t265: f64, t504: f64, t89823: f64, t12606: f64, t1409: f64, t2161: f64, t2250: f64, t24916: f64, t27850: f64, t52: f64, t607: f64, t7402: f64, t8097: f64, t90003: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64, t113: f64, t12545: f64, t12835: f64, t1393: f64, t24932: f64, t27903: f64, t4077: f64, t7266: f64, t91602: f64, t91606: f64, t91608: f64, t91610: f64, t91612: f64, t91623: f64, t91625: f64, t91627: f64, t91630: f64, t91637: f64, t91640: f64, t91642: f64, t91657: f64, t91662: f64, t94293: f64) -> f64 {
        let t95703 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2326(t24682, t460, t95484, t27634, t3030, t86259, t24740, t5064, t15640, t24729, t14726, t15394, t2121, t2132, t2133, t24706, t27639, t27645, t27674, t27704, t3552, t3557, t3580, t4928, t7321, t7331, t86365, t86368);
        let t95707 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2327(t95260, t95285, t95316, t95343, t95367, t95407, t95443, t95469, t95492, t95518, t95543, t95576, t95603, t95633, t95672, t95703);
        let t95723 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2328(t24574, t27574, t24844, t7999, t1244, t1246, t15015, t15027, t1729, t24792, t24863, t27470, t27724, t3471, t3493, t3624, t470, t493, t5079, t7283, t7373, t7375, t7376, t8054, t8077, t86020, t95707);
        let t95752 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2329(t2121, t3427, t8077, t27517, t85639, t24574, t27481, t11888, t11904, t15022, t15247, t24589, t24794, t24798, t24841, t24849, t27516, t27532, t27543, t3565, t3624, t5064, t5072, t7327, t8082, t8085, t86057);
        let (t95772, t95779) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2330(t7365, t94490, t1715, t974, t24847, t24771, t7999, t15418, t2127, t221, t27553, t11877, t11907, t11914, t15245, t15429, t24765, t24834, t24838, t27406, t27454, t27546, t7283, t8082, t8083, t86073, t86095, t94588);
        let t95817 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2331(t477, t5052, t27654, t7327, t24745, t4935, t1090, t1186, t1201, t1215, t15771, t2121, t2147, t24589, t24799, t24849, t24851, t27406, t27525, t27549, t27552, t27722, t27732, t3966, t462, t7283, t7362, t7364, t7373, t7376, t7377, t86106, t86113, t86116, t94976);
        let t95844 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2332(t24585, t7999, t24574, t27800, t225, t27805, t11613, t1191, t1238, t1241, t1252, t15802, t1720, t2155, t24612, t24757, t24897, t254, t27784, t27785, t27786, t27792, t3631, t4940, t498, t5055, t53703, t7348, t8088, t94779, t94820, t94867, t94902, t94942, t94980, t95026, t95058, t95087, t95122, t95150, t95184, t95224, t95723, t95752, t95779, t95817);
        let t95876 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2333(t24574, t27392, t1170, t2121, t27766, t1238, t15794, t1716, t24567, t24568, t24582, t24630, t24639, t24877, t24893, t27406, t27415, t3598, t3630, t4945, t5055, t5060, t7283, t7351, t8087, t86473, t86494);
        let t95913 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2334(t2154, t45349, t27776, t95772, t11147, t497, t225, t27424, t27422, t24574, t27752, t1252, t14165, t15787, t15793, t24601, t24888, t27406, t27784, t27830, t3471, t3631, t466, t498, t7283, t7300, t7351, t8002, t8010, t85674, t85750, t86501, t94796, t95707);
        let t95952 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2335(t27834, t3640, t11947, t8090, t1254, t1256, t15834, t1763, t193, t24905, t24909, t27838, t27843, t336, t3633, t3637, t4700, t5091, t64447, t7398, t86513, t86517, t86524, t94341, t94385, t94428, t94464, t94498, t94530, t94564, t94605, t94637, t94673, t94698, t94734, t94770, t95844, t95876, t95913);
        let t95965 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2336(t28, t265, t504, t89823, t95952, t12606, t1409, t2161, t2250, t24916, t27850, t3966, t52, t607, t7402, t8097, t90003, dens_threshold, rho1, zeta_threshold);
        let t95970 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2337(t113, t12545, t12835, t1393, t24932, t27903, t4077, t7266, t91602, t91606, t91608, t91610, t91612, t91623, t91625, t91627, t91630, t91637, t91640, t91642, t91657, t91662, t94293, t95965);
    t95970
}
