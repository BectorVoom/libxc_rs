//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta863 (260520-c91 hierarchical CSE).
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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3141;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3142;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3143;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3144;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3145;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3146;
use chunk6::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3147;
use chunk7::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3148;
use chunk8::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3149;
use chunk9::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3150;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta863(t11570: f64, t17686: f64, t1174: f64, t15269: f64, t15274: f64, t15288: f64, t18420: f64, t3447: f64, t3449: f64, t3469: f64, t44487: f64, t460: f64, t4889: f64, t4900: f64, t4934: f64, t6138: f64, t64969: f64, t64976: f64, t64979: f64, t64981: f64, t64988: f64, t64990: f64, t15299: f64, t15285: f64, t12652: f64, t14725: f64, t44505: f64, t15363: f64, t15281: f64, t18549: f64, t1090: f64, t1184: f64, t15304: f64, t15376: f64, t15383: f64, t15395: f64, t18523: f64, t27654: f64, t3440: f64, t3441: f64, t44504: f64, t4919: f64, t52191: f64, t55723: f64, t18554: f64, t17635: f64, t11569: f64, t1177: f64, t1178: f64, t15390: f64, t18321: f64, t3443: f64, t3457: f64, t3461: f64, t3475: f64, t52066: f64, t52100: f64, t52224: f64, t52228: f64, t52240: f64, t52250: f64, t55677: f64, t11583: f64, t17691: f64, t15372: f64, t11529: f64, t6126: f64, t15278: f64, t15357: f64, t15360: f64, t18416: f64, t52216: f64, t52220: f64, t6144: f64, t8034: f64, t44571: f64, t6119: f64, t44607: f64, t15382: f64, t52059: f64, t15338: f64, t18542: f64, t15293: f64, t15289: f64, t15320: f64, t3455: f64, t52140: f64, t52281: f64, t52288: f64, t52296: f64, t15294: f64, t44573: f64, t44586: f64, t44635: f64, t44638: f64, t44641: f64, t52300: f64, t52354: f64, t52357: f64, t52362: f64, t52364: f64, t52367: f64, t64634: f64, t64660: f64, t64694: f64, t64725: f64, t64746: f64, t64786: f64, t64823: f64, t64845: f64, t64883: f64, t64966: f64, t19256: f64, t225: f64, t11606: f64, t11613: f64, t1190: f64, t1238: f64, t1252: f64, t15787: f64, t15794: f64, t15820: f64, t1761: f64, t19120: f64, t19214: f64, t19226: f64, t19232: f64, t3487: f64, t3593: f64, t3598: f64, t3599: f64, t3600: f64, t3630: f64, t491: f64, t4945: f64, t498: f64, t5055: f64, t5089: f64, t51937: f64, t52386: f64, t6243: f64, t6244: f64, t6267: f64, t19211: f64, t3507: f64, t6238: f64, t11914: f64, t1244: f64, t1246: f64, t14997: f64, t15022: f64, t15023: f64, t15027: f64, t15239: f64, t15245: f64, t15430: f64, t15771: f64, t15777: f64, t1734: f64, t1751: f64, t1755: f64, t19138: f64, t19166: f64, t19190: f64, t3493: f64, t3604: f64, t3624: f64, t3625: f64, t45326: f64, t475: f64, t5064: f64, t5072: f64, t53592: f64, t6252: f64, t6260: f64, t6739: f64, t3030: f64, t6150: f64, t3609: f64, t3623: f64, t5011: f64, t63280: f64, t64446: f64, t64454: f64, t64456: f64, t64458: f64, t64460: f64, t64462: f64, t64464: f64, t64466: f64, t64470: f64, t64472: f64, t64475: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t65001 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3141(t11570, t17686, t1174, t15269, t15274, t15288, t18420, t3447, t3449, t3469, t44487, t460, t4889, t4900, t4934, t6138, t64969, t64976, t64979, t64981, t64988, t64990);
        let (t65014, t65037) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3142(t15299, t4889, t15285, t12652, t14725, t17686, t44505, t15363, t1174, t15281, t18549, t1090, t1184, t15304, t15376, t15383, t15395, t18523, t27654, t3440, t3441, t3447, t44504, t460, t4919, t4934, t52191, t55723);
        let t65073 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3143(t1174, t15281, t18554, t11570, t17635, t11569, t1177, t1178, t15390, t18321, t3443, t3447, t3457, t3461, t3475, t460, t4919, t4934, t52066, t52100, t52224, t52228, t52240, t52250, t55677, t6138);
        let t65114 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3144(t11583, t17635, t11570, t17691, t15372, t4889, t11529, t1174, t6126, t11569, t15278, t15288, t15357, t15360, t18416, t3447, t3449, t3469, t3475, t460, t4919, t4934, t52216, t52220, t6144, t8034);
        let t65147 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3145(t1174, t44571, t6119, t17686, t44607, t15382, t3447, t52059, t15338, t18542, t15293, t11569, t1177, t15289, t15320, t15376, t3455, t52140, t52281, t52288, t52296, t55723);
        let t65161 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3146(t15294, t15376, t44573, t44586, t44635, t44638, t44641, t52300, t52354, t52357, t52362, t52364, t52367);
        let t65165 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3147(t64634, t64660, t64694, t64725, t64746, t64786, t64823, t64845, t64883, t64966, t65001, t65037, t65073, t65114, t65147, t65161);
        let t65206 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3148(t19256, t225, t11606, t11613, t1190, t1238, t1252, t15787, t15794, t15820, t1761, t19120, t19214, t19226, t19232, t3487, t3593, t3598, t3599, t3600, t3630, t491, t4945, t498, t5055, t5089, t51937, t52386, t6243, t6244, t6267, t65165);
        let (t65208, t65221, t65249) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3149(t19211, t225, t3507, t6238, t11914, t1244, t1246, t14997, t15022, t15023, t15027, t15239, t15245, t15430, t15771, t15777, t1734, t1751, t1755, t19138, t19166, t19190, t3493, t3604, t3624, t3625, t45326, t475, t5064, t5072, t53592, t6252, t6260, t6739);
        let (t65253, t65254, t65262, t65264, t65265, t65278) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3150(t3030, t6150, t3609, t3623, t5011, t491, t63280, t64446, t64454, t64456, t64458, t64460, t64462, t64464, t64466, t64470, t64472, t64475);
    (t65014, t65165, t65206, t65208, t65221, t65249, t65253, t65254, t65262, t65264, t65265, t65278)
}
