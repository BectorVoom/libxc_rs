//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta681 (260520-c91 hierarchical CSE).
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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2293;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2294;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2295;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2296;
use chunk4::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2297;
use chunk5::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2298;
use chunk6::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2299;
use chunk7::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2300;
use chunk8::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2301;
use chunk9::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2302;
use chunk10::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2303;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta681(t27495: f64, t85964: f64, t1734: f64, t3032: f64, t15702: f64, t8038: f64, t85822: f64, t27563: f64, t85639: f64, t24826: f64, t27502: f64, t27558: f64, t7368: f64, t94490: f64, t15359: f64, t15661: f64, t1755: f64, t2148: f64, t24660: f64, t24807: f64, t24815: f64, t24830: f64, t27507: f64, t3516: f64, t4930: f64, t7283: f64, t7381: f64, t7999: f64, t85820: f64, t85963: f64, t86037: f64, t1193: f64, t27506: f64, t7378: f64, t11153: f64, t491: f64, t8034: f64, t24667: f64, t27537: f64, t12648: f64, t12652: f64, t14165: f64, t14985: f64, t24781: f64, t24784: f64, t24804: f64, t24806: f64, t24812: f64, t24816: f64, t24822: f64, t27406: f64, t27536: f64, t27549: f64, t27550: f64, t27551: f64, t5064: f64, t7373: f64, t7375: f64, t7376: f64, t27526: f64, t86094: f64, t24850: f64, t1409: f64, t3507: f64, t24847: f64, t64825: f64, t974: f64, t8067: f64, t85660: f64, t2147: f64, t7319: f64, t11871: f64, t15032: f64, t24589: f64, t24821: f64, t24859: f64, t27516: f64, t27562: f64, t3610: f64, t7387: f64, t8082: f64, t85824: f64, t85854: f64, t86076: f64, t86077: f64, t94850: f64, t1011: f64, t5011: f64, t11715: f64, t27488: f64, t1209: f64, t1216: f64, t1235: f64, t15018: f64, t15620: f64, t15625: f64, t24762: f64, t24813: f64, t24814: f64, t24833: f64, t24834: f64, t27470: f64, t27471: f64, t27489: f64, t27490: f64, t27496: f64, t27497: f64, t27501: f64, t3494: f64, t3509: f64, t3604: f64, t5068: f64, t8070: f64, t225: f64, t27654: f64, t24574: f64, t27484: f64, t1244: f64, t1246: f64, t15426: f64, t2152: f64, t24776: f64, t24820: f64, t24849: f64, t27460: f64, t27510: f64, t27532: f64, t3243: f64, t5075: f64, t7327: f64, t7348: f64, t7364: f64, t85883: f64, t85918: f64, t27540: f64, t14706: f64, t27478: f64, t27491: f64, t27724: f64, t3477: f64, t3502: f64, t4978: f64, t7362: f64, t7363: f64, t8077: f64, t85941: f64, t85943: f64, t85945: f64, t85952: f64, t85955: f64, t210: f64, t24848: f64, t27505: f64, t27466: f64, t8054: f64, t27455: f64, t24851: f64, t24853: f64, t24860: f64, t27725: f64, t3248: f64, t3252: f64, t3493: f64, t3612: f64, t85984: f64, t85986: f64, t27474: f64, t27492: f64, t85853: f64, t27498: f64, t1215: f64, t15239: f64, t2144: f64, t24858: f64, t27520: f64, t27721: f64, t3624: f64, t3625: f64, t4733: f64, t8073: f64, t85920: f64, t85988: f64, t85996: f64, t86000: f64, t27533: f64, t27521: f64, t24745: f64, t24757: f64, t24777: f64, t24788: f64, t27453: f64, t27454: f64, t27465: f64, t3242: f64, t3961: f64, t8066: f64, t85832: f64, t86001: f64, t94400: f64, t94404: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t94874, t94875, t94881, t94885, t94889, t94891) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2293(t27495, t85964, t1734, t3032, t15702, t8038, t85822, t27563, t85639, t24826, t27502, t27558);
        let t94902 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2294(t7368, t94490, t15359, t15661, t1755, t2148, t24660, t24807, t24815, t24830, t27507, t3516, t4930, t7283, t7381, t7999, t85820, t85963, t86037, t94874, t94875, t94881, t94885, t94889, t94891);
        let t94942 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2295(t1193, t27506, t7378, t11153, t491, t24660, t8034, t24667, t24826, t27537, t12648, t12652, t14165, t14985, t24781, t24784, t24804, t24806, t24812, t24816, t24822, t27406, t27536, t27549, t27550, t27551, t5064, t7373, t7375, t7376);
        let (t94947, t94948, t94949, t94954, t94963, t94966) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2296(t27526, t86094, t24660, t24850, t1409, t3507, t24667, t24847, t64825, t974, t8067, t85660);
        let (t94976, t94980) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2297(t2147, t7319, t11871, t15032, t24589, t24815, t24821, t24859, t27516, t27562, t3610, t7387, t8082, t85824, t85854, t86037, t86076, t86077, t94850, t94947, t94948, t94949, t94954, t94963, t94966);
        let (t94986, t95026) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2298(t1011, t5011, t11715, t491, t85964, t27488, t1209, t1216, t1235, t15018, t15620, t15625, t24762, t24812, t24813, t24814, t24815, t24833, t24834, t27406, t27470, t27471, t27489, t27490, t27496, t27497, t27501, t27507, t3494, t3509, t3604, t3610, t5068, t7373, t85963, t94875);
        let t95058 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2299(t8070, t85660, t225, t27654, t24574, t27484, t1244, t1246, t15018, t15426, t2152, t24589, t24776, t24812, t24820, t24821, t24833, t24849, t27460, t27510, t27532, t3243, t5011, t5075, t7283, t7327, t7348, t7364, t7373, t85883, t85918);
        let t95087 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2300(t24826, t27540, t1235, t14706, t24812, t24813, t27478, t27489, t27491, t27724, t3477, t3502, t3604, t3610, t4978, t5068, t7283, t7362, t7363, t8077, t85941, t85943, t85945, t85952, t85955, t94986);
        let (t95109, t95122) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2301(t210, t24848, t27505, t24574, t27466, t3507, t8054, t27455, t1409, t24849, t24851, t24853, t24860, t27406, t27460, t27725, t3248, t3252, t3493, t3604, t3610, t3612, t7283, t7362, t7376, t85984, t85986);
        let t95150 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2302(t24574, t27474, t27492, t85853, t27498, t1215, t1244, t1246, t15239, t2144, t24833, t24858, t27520, t27721, t3624, t3625, t4733, t7283, t7362, t7373, t8073, t85920, t85988, t85996, t86000, t95109);
        let t95184 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2303(t27533, t86094, t24826, t27521, t1235, t1244, t1246, t1734, t24589, t24745, t24757, t24777, t24788, t24858, t27453, t27454, t27465, t27516, t27549, t27550, t3242, t3961, t7283, t8066, t85832, t86001, t94400, t94404);
    (t94902, t94942, t94976, t94980, t95026, t95058, t95087, t95122, t95150, t95184)
}
