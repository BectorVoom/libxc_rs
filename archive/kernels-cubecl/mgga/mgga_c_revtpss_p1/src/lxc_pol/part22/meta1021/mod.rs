//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta1021 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3549;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3550;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3551;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3552;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3553;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3554;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3555;
use chunk7::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3556;
use chunk8::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3557;
use chunk9::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3558;
use chunk10::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3559;
use chunk11::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3560;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1021<F: Float>(t1043: F, t19380: F, t1086: F, t19462: F, t3090: F, t11922: F, t19972: F, t4892: F, t19658: F, t3124: F, t19882: F, t3106: F, t1063: F, t11986: F, t247: F, t6096: F, t1045: F, t15785: F, t19572: F, t3115: F, t3117: F, t3120: F, t4894: F, t55293: F, t55320: F, t55325: F, t55328: F, t55361: F, t55367: F, t66565: F, t65316: F, t65353: F, t65438: F, t65468: F, t65497: F, t65533: F, t65563: F, t65591: F, t65626: F, t65659: F, t65693: F, t65727: F, t65753: F, t65795: F, t65819: F, t65852: F, t65888: F, t65929: F, t65973: F, t66013: F, t66054: F, t66086: F, t66127: F, t66161: F, t66204: F, t66227: F, t66263: F, t66294: F, t66336: F, t66373: F, t66414: F, t66460: F, t66500: F, t66535: F, t66558: F, t66591: F, t66631: F, t66662: F, t66682: F, t66716: F, t66749: F, t66793: F, t66827: F, t66865: F, t66893: F, t66925: F, t66956: F, t66997: F, t67031: F, t67058: F, t67102: F, t67143: F, t67182: F, t67218: F, t67257: F, t67283: F, t67318: F, t67345: F, t67382: F, t67430: F, t67470: F, t67509: F, t67543: F, t20112: F, t359: F, t3302: F, t3259: F, t6305: F, t1024: F, t1082: F, t11940: F, t12122: F, t12127: F, t15604: F, t15609: F, t15717: F, t16409: F, t16410: F, t1647: F, t16505: F, t16566: F, t19447: F, t19456: F, t19521: F, t19566: F, t19594: F, t19597: F, t3204: F, t3291: F, t3299: F, t3304: F, t3309: F, t3322: F, t342: F, t380: F, t43360: F, t43453: F, t43598: F, t4984: F, t4999: F, t5004: F, t55499: F, t6235: F, t64831: F, t66771: F, t999: F, t12046: F, t1678: F, t6343: F, t994: F, t4772: F, t4975: F, t12050: F, t19450: F, t1089: F, t12149: F, t16396: F, t16405: F, t16432: F, t16569: F, t16573: F, t16581: F, t19446: F, t19503: F, t19580: F, t19603: F, t19829: F, t20139: F, t3287: F, t3288: F, t43357: F, t43443: F, t43446: F, t43520: F, t43528: F, t4857: F, t4905: F, t4976: F, t4996: F, t6365: F, t65773: F, t66382: F, t73: F, t3286: F, t12154: F, t16393: F, t16468: F, t16506: F, t16544: F, t16559: F, t16560: F, t16574: F, t19453: F, t19484: F, t19492: F, t19549: F, t19557: F, t19569: F, t19602: F, t19607: F, t19608: F, t19612: F, t3133: F, t3223: F, t3317: F, t3318: F, t4964: F, t55575: F, t55632: F, t55934: F, t55944: F, t989: F, t3298: F, t378: F, t65481: F, t11788: F, t12097: F, t16183: F, t16406: F, t16440: F, t16465: F, t16534: F, t19482: F, t19509: F, t19836: F, t20146: F, t3151: F, t3305: F, t43432: F, t4893: F, t4954: F, t55569: F, t55570: F, t55593: F, t55594: F, t6383: F, t64891: F, t3316: F, t1071: F, t1087: F, t12132: F, t12146: F, t16390: F, t16433: F, t16436: F, t16502: F, t19463: F, t19477: F, t19498: F, t19501: F, t19593: F, t3059: F, t3075: F, t3283: F, t3292: F, t3319: F, t43456: F, t43611: F, t55732: F, t56049: F, t6386: F, t65425: F, t66341: F, t66395: F, t67501: F, t19856: F, t3153: F, t1090: F, t12073: F, t15670: F, t16461: F, t16482: F, t16537: F, t16540: F, t16568: F, t16577: F, t19611: F, t20128: F, t43341: F, t4977: F, t4981: F, t4983: F, t4998: F, t55646: F, t55988: F, t55991: F, t6258: F, t64861: F, t67438: F, t12160: F, t16237: F, t16427: F, t16509: F, t16552: F, t16554: F, t16561: F, t1668: F, t19534: F, t20119: F, t20123: F, t3278: F, t3313: F, t42359: F, t43524: F, t6362: F, t65144: F, t66945: F, t16543: F, t4746: F, t16443: F, t16523: F, t16578: F, t19512: F, t20136: F, t3295: F, t55701: F, t55747: F, t55887: F, t65881: F, t16551: F, t16558: F, t12116: F, t16381: F, t16520: F, t16529: F, t16555: F, t16562: F, t19438: F, t19526: F, t19539: F, t19573: F, t19576: F, t20113: F, t4866: F, t4930: F, t4982: F, t4988: F, t4992: F, t5009: F, t6299: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t67545, t67551, t67560, t67568, t67571) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3549::<F>(t1043, t19380, t1086, t19462, t3090, t11922, t19972, t4892, t19658, t3124, t19882, t3106);
        let t67578 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3550::<F>(t1063, t11986, t247, t6096, t1045, t15785, t19572, t3115, t3117, t3120, t4892, t4894, t55293, t55320, t55325, t55328, t55361, t55367, t66565, t67545, t67551, t67560, t67568, t67571);
        let t67584 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3551::<F>(t65316, t65353, t65438, t65468, t65497, t65533, t65563, t65591, t65626, t65659, t65693, t65727, t65753, t65795, t65819, t65852, t65888, t65929, t65973, t66013, t66054, t66086, t66127, t66161, t66204, t66227, t66263, t66294, t66336, t66373, t66414, t66460, t66500, t66535, t66558, t66591, t66631, t66662, t66682, t66716, t66749, t66793, t66827, t66865, t66893, t66925, t66956, t66997, t67031, t67058, t67102, t67143, t67182, t67218, t67257, t67283, t67318, t67345, t67382, t67430, t67470, t67509, t67543, t67578);
        let (t67618, t67633) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3552::<F>(t20112, t359, t19572, t3302, t3259, t6305, t1024, t1082, t11940, t12122, t12127, t15604, t15609, t15717, t16409, t16410, t1647, t16505, t16566, t19447, t19456, t19521, t19566, t19594, t19597, t3204, t3291, t3299, t3304, t3309, t3322, t342, t380, t43360, t43453, t43598, t4984, t4999, t5004, t55499, t6235, t64831, t66771, t67584, t999);
        let (t67678, t67684) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3553::<F>(t12046, t1678, t342, t1086, t6343, t994, t4772, t4975, t12050, t19450, t1089, t12127, t12149, t16396, t16405, t16432, t16569, t16573, t16581, t19446, t19447, t19503, t19572, t19580, t19603, t19829, t20139, t3287, t3288, t43357, t43360, t43443, t43446, t43520, t43528, t43598, t4857, t4905, t4976, t4996, t6365, t65773, t66382, t67545, t73);
        let t67723 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3554::<F>(t19462, t3286, t12154, t16393, t16468, t16506, t16544, t16559, t16560, t16574, t19450, t19453, t19484, t19492, t19549, t19557, t19569, t19602, t19607, t19608, t19612, t3133, t3223, t3288, t3317, t3318, t4964, t4984, t4999, t55575, t55632, t55934, t55944, t67618, t989);
        let (t67748, t67768) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3555::<F>(t3298, t6235, t378, t65481, t11788, t12097, t12149, t12154, t16183, t16406, t16440, t16465, t16534, t16544, t19482, t19509, t19594, t19597, t19608, t19836, t20139, t20146, t3151, t3305, t3317, t3318, t43432, t43443, t43528, t4893, t4954, t4976, t4996, t55569, t55570, t55593, t55594, t6383, t64891, t73);
        let t67813 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3556::<F>(t3316, t6235, t1071, t1087, t1089, t12122, t12127, t12132, t12146, t16390, t16433, t16436, t16502, t19463, t19477, t19482, t19498, t19501, t19593, t19612, t3059, t3075, t3283, t3287, t3292, t3318, t3319, t378, t43456, t43611, t4976, t55732, t56049, t6386, t65425, t66341, t66395, t67501);
        let t67859 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3557::<F>(t1086, t19856, t19836, t3153, t1024, t1089, t1090, t12073, t12122, t12127, t12149, t15670, t16461, t16482, t16537, t16540, t16568, t16577, t19611, t20128, t3223, t3287, t3299, t3304, t43341, t4857, t4977, t4981, t4983, t4998, t55646, t55988, t55991, t6258, t64861, t66341, t66565, t67438, t67748);
        let (t67869, t67905) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3558::<F>(t3151, t6343, t1043, t1087, t1089, t12160, t15604, t15609, t16183, t16237, t16427, t16433, t16509, t16552, t16554, t16559, t16561, t1668, t1678, t19534, t19566, t20112, t20119, t20123, t3278, t3299, t3304, t3313, t42359, t43341, t43524, t55499, t55988, t6362, t65144, t66945, t67678);
        let t67946 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3559::<F>(t16543, t4746, t1087, t1089, t11788, t12122, t12154, t16405, t16406, t16432, t16443, t16502, t16523, t16578, t19463, t19484, t19503, t19512, t19603, t19611, t20136, t3133, t3287, t3288, t3295, t3317, t3318, t43432, t4964, t4996, t4998, t55701, t55747, t55887, t6343, t65881, t66565, t67869);
        let t67989 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3560::<F>(t1647, t16551, t16558, t1087, t1089, t12116, t12122, t16381, t16393, t16443, t16502, t16520, t16523, t16529, t16555, t16562, t19438, t19501, t19526, t19539, t19573, t19576, t20113, t3075, t3223, t3259, t4866, t4930, t4977, t4982, t4988, t4992, t5009, t55934, t6299, t989);
    (t67584, t67633, t67684, t67723, t67768, t67813, t67859, t67905, t67946, t67989)
}
