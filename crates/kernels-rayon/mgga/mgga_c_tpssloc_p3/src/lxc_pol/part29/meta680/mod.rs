//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta680 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2286;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2287;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2288;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2289;
use chunk4::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2290;
use chunk5::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2291;
use chunk6::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2292;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta680(t24574: f64, t27383: f64, t7288: f64, t94490: f64, t11613: f64, t1190: f64, t15820: f64, t24634: f64, t24880: f64, t24883: f64, t24887: f64, t27406: f64, t27426: f64, t27721: f64, t27742: f64, t27747: f64, t3481: f64, t3487: f64, t3593: f64, t498: f64, t5089: f64, t7283: f64, t7356: f64, t8054: f64, t8061: f64, t86390: f64, t27438: f64, t85639: f64, t225: f64, t27419: f64, t1236: f64, t1252: f64, t12652: f64, t1409: f64, t15797: f64, t2128: f64, t24589: f64, t24590: f64, t24601: f64, t24602: f64, t24626: f64, t24638: f64, t24877: f64, t254: f64, t27388: f64, t27444: f64, t27786: f64, t3630: f64, t4936: f64, t4945: f64, t7392: f64, t27427: f64, t5052: f64, t7284: f64, t14980: f64, t15803: f64, t1761: f64, t2155: f64, t24868: f64, t27382: f64, t3477: f64, t5055: f64, t51928: f64, t7287: f64, t7351: f64, t86400: f64, t86409: f64, t86424: f64, t27779: f64, t8015: f64, t85660: f64, t27826: f64, t11606: f64, t11925: f64, t1238: f64, t15771: f64, t15789: f64, t2121: f64, t24564: f64, t24591: f64, t27549: f64, t27774: f64, t27784: f64, t27785: f64, t27792: f64, t3598: f64, t3599: f64, t3600: f64, t462: f64, t497: f64, t5088: f64, t53658: f64, t7391: f64, t8087: f64, t8088: f64, t86426: f64, t94395: f64, t1751: f64, t24594: f64, t27403: f64, t1251: f64, t14706: f64, t15425: f64, t15786: f64, t1716: f64, t2144: f64, t2154: f64, t24596: f64, t24893: f64, t27741: f64, t4930: f64, t5060: f64, t51925: f64, t7285: f64, t7286: f64, t85688: f64, t86451: f64, t86456: f64, t27389: f64, t8074: f64, t85917: f64, t24826: f64, t27511: f64, t15394: f64, t2127: f64, t221: f64, t11147: f64, t491: f64, t1235: f64, t12648: f64, t14165: f64, t14988: f64, t15240: f64, t24788: f64, t24789: f64, t27461: f64, t27473: f64, t27550: f64, t27561: f64, t3247: f64, t3961: f64, t7373: f64, t7375: f64, t7376: f64, t1089: f64, t7327: f64, t1653: f64, t7330: f64, t85822: f64, t131: f64, t1419: f64, t23598: f64, t467: f64, t15702: f64, t15776: f64, t1755: f64, t24667: f64, t24785: f64, t24817: f64, t24823: f64, t24849: f64, t24852: f64, t27507: f64, t27531: f64, t27551: f64, t27643: f64, t3248: f64, t3252: f64, t8066: f64, t85820: f64, t86015: f64, t86037: f64, t86059: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t94637 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2286(t24574, t27383, t7288, t94490, t11613, t1190, t15820, t24634, t24880, t24883, t24887, t27406, t27426, t27721, t27742, t27747, t3481, t3487, t3593, t498, t5089, t7283, t7356, t8054, t8061, t86390);
        let t94673 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2287(t27438, t85639, t225, t27419, t1236, t1252, t12652, t1409, t15797, t15820, t2128, t24589, t24590, t24601, t24602, t24626, t24638, t24877, t254, t27388, t27406, t27444, t27747, t27786, t3487, t3630, t4936, t4945, t7356, t7392);
        let t94698 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2288(t24574, t27427, t5052, t7284, t14980, t15803, t1761, t2155, t24868, t27382, t27742, t3477, t3593, t4945, t5055, t51928, t7283, t7287, t7351, t7356, t7392, t86400, t86409, t86424);
        let t94734 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2289(t24574, t27779, t8015, t85660, t27826, t11606, t11925, t1238, t12652, t15771, t15789, t2121, t2155, t225, t24564, t24591, t24601, t27406, t27549, t27774, t27784, t27785, t27792, t3598, t3599, t3600, t462, t497, t5088, t53658, t7391, t8087, t8088, t86426, t94395);
        let t94770 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2290(t1751, t24594, t24574, t27403, t1238, t1251, t14706, t15425, t15786, t1716, t2144, t2154, t2155, t24596, t24638, t24880, t24893, t27741, t3598, t4930, t498, t5060, t5089, t51925, t7283, t7285, t7286, t85688, t86451, t86456);
        let (t94779, t94796, t94820) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2291(t24574, t27389, t8074, t85917, t24826, t27511, t15394, t2127, t221, t11147, t491, t1235, t12648, t12652, t14165, t14988, t15240, t24589, t24788, t24789, t27461, t27473, t27550, t27561, t3247, t3961, t7373, t7375, t7376, t94395);
        let (t94850, t94867) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2292(t1089, t1751, t7327, t1653, t7330, t85822, t3961, t131, t1419, t23598, t467, t14165, t15702, t15776, t1755, t24589, t24667, t24785, t24817, t24823, t24849, t24852, t27507, t27531, t27550, t27551, t27643, t3248, t3252, t7373, t7375, t7376, t8066, t85820, t86015, t86037, t86059);
    (t94637, t94673, t94698, t94734, t94770, t94779, t94796, t94820, t94850, t94867)
}
