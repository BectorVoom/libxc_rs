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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

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
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta681<F: Float>(t27495: F, t85964: F, t1734: F, t3032: F, t15702: F, t8038: F, t85822: F, t27563: F, t85639: F, t24826: F, t27502: F, t27558: F, t7368: F, t94490: F, t15359: F, t15661: F, t1755: F, t2148: F, t24660: F, t24807: F, t24815: F, t24830: F, t27507: F, t3516: F, t4930: F, t7283: F, t7381: F, t7999: F, t85820: F, t85963: F, t86037: F, t1193: F, t27506: F, t7378: F, t11153: F, t491: F, t8034: F, t24667: F, t27537: F, t12648: F, t12652: F, t14165: F, t14985: F, t24781: F, t24784: F, t24804: F, t24806: F, t24812: F, t24816: F, t24822: F, t27406: F, t27536: F, t27549: F, t27550: F, t27551: F, t5064: F, t7373: F, t7375: F, t7376: F, t27526: F, t86094: F, t24850: F, t1409: F, t3507: F, t24847: F, t64825: F, t974: F, t8067: F, t85660: F, t2147: F, t7319: F, t11871: F, t15032: F, t24589: F, t24821: F, t24859: F, t27516: F, t27562: F, t3610: F, t7387: F, t8082: F, t85824: F, t85854: F, t86076: F, t86077: F, t94850: F, t1011: F, t5011: F, t11715: F, t27488: F, t1209: F, t1216: F, t1235: F, t15018: F, t15620: F, t15625: F, t24762: F, t24813: F, t24814: F, t24833: F, t24834: F, t27470: F, t27471: F, t27489: F, t27490: F, t27496: F, t27497: F, t27501: F, t3494: F, t3509: F, t3604: F, t5068: F, t8070: F, t225: F, t27654: F, t24574: F, t27484: F, t1244: F, t1246: F, t15426: F, t2152: F, t24776: F, t24820: F, t24849: F, t27460: F, t27510: F, t27532: F, t3243: F, t5075: F, t7327: F, t7348: F, t7364: F, t85883: F, t85918: F, t27540: F, t14706: F, t27478: F, t27491: F, t27724: F, t3477: F, t3502: F, t4978: F, t7362: F, t7363: F, t8077: F, t85941: F, t85943: F, t85945: F, t85952: F, t85955: F, t210: F, t24848: F, t27505: F, t27466: F, t8054: F, t27455: F, t24851: F, t24853: F, t24860: F, t27725: F, t3248: F, t3252: F, t3493: F, t3612: F, t85984: F, t85986: F, t27474: F, t27492: F, t85853: F, t27498: F, t1215: F, t15239: F, t2144: F, t24858: F, t27520: F, t27721: F, t3624: F, t3625: F, t4733: F, t8073: F, t85920: F, t85988: F, t85996: F, t86000: F, t27533: F, t27521: F, t24745: F, t24757: F, t24777: F, t24788: F, t27453: F, t27454: F, t27465: F, t3242: F, t3961: F, t8066: F, t85832: F, t86001: F, t94400: F, t94404: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t94874, t94875, t94881, t94885, t94889, t94891) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2293::<F>(t27495, t85964, t1734, t3032, t15702, t8038, t85822, t27563, t85639, t24826, t27502, t27558);
        let t94902 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2294::<F>(t7368, t94490, t15359, t15661, t1755, t2148, t24660, t24807, t24815, t24830, t27507, t3516, t4930, t7283, t7381, t7999, t85820, t85963, t86037, t94874, t94875, t94881, t94885, t94889, t94891);
        let t94942 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2295::<F>(t1193, t27506, t7378, t11153, t491, t24660, t8034, t24667, t24826, t27537, t12648, t12652, t14165, t14985, t24781, t24784, t24804, t24806, t24812, t24816, t24822, t27406, t27536, t27549, t27550, t27551, t5064, t7373, t7375, t7376);
        let (t94947, t94948, t94949, t94954, t94963, t94966) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2296::<F>(t27526, t86094, t24660, t24850, t1409, t3507, t24667, t24847, t64825, t974, t8067, t85660);
        let (t94976, t94980) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2297::<F>(t2147, t7319, t11871, t15032, t24589, t24815, t24821, t24859, t27516, t27562, t3610, t7387, t8082, t85824, t85854, t86037, t86076, t86077, t94850, t94947, t94948, t94949, t94954, t94963, t94966);
        let (t94986, t95026) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2298::<F>(t1011, t5011, t11715, t491, t85964, t27488, t1209, t1216, t1235, t15018, t15620, t15625, t24762, t24812, t24813, t24814, t24815, t24833, t24834, t27406, t27470, t27471, t27489, t27490, t27496, t27497, t27501, t27507, t3494, t3509, t3604, t3610, t5068, t7373, t85963, t94875);
        let t95058 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2299::<F>(t8070, t85660, t225, t27654, t24574, t27484, t1244, t1246, t15018, t15426, t2152, t24589, t24776, t24812, t24820, t24821, t24833, t24849, t27460, t27510, t27532, t3243, t5011, t5075, t7283, t7327, t7348, t7364, t7373, t85883, t85918);
        let t95087 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2300::<F>(t24826, t27540, t1235, t14706, t24812, t24813, t27478, t27489, t27491, t27724, t3477, t3502, t3604, t3610, t4978, t5068, t7283, t7362, t7363, t8077, t85941, t85943, t85945, t85952, t85955, t94986);
        let (t95109, t95122) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2301::<F>(t210, t24848, t27505, t24574, t27466, t3507, t8054, t27455, t1409, t24849, t24851, t24853, t24860, t27406, t27460, t27725, t3248, t3252, t3493, t3604, t3610, t3612, t7283, t7362, t7376, t85984, t85986);
        let t95150 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2302::<F>(t24574, t27474, t27492, t85853, t27498, t1215, t1244, t1246, t15239, t2144, t24833, t24858, t27520, t27721, t3624, t3625, t4733, t7283, t7362, t7373, t8073, t85920, t85988, t85996, t86000, t95109);
        let t95184 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2303::<F>(t27533, t86094, t24826, t27521, t1235, t1244, t1246, t1734, t24589, t24745, t24757, t24777, t24788, t24858, t27453, t27454, t27465, t27516, t27549, t27550, t3242, t3961, t7283, t8066, t85832, t86001, t94400, t94404);
    (t94902, t94942, t94976, t94980, t95026, t95058, t95087, t95122, t95150, t95184)
}
