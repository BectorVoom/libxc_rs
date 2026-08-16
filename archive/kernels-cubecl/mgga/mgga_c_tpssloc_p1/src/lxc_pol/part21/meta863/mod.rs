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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

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
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta863<F: Float>(t11570: F, t17686: F, t1174: F, t15269: F, t15274: F, t15288: F, t18420: F, t3447: F, t3449: F, t3469: F, t44487: F, t460: F, t4889: F, t4900: F, t4934: F, t6138: F, t64969: F, t64976: F, t64979: F, t64981: F, t64988: F, t64990: F, t15299: F, t15285: F, t12652: F, t14725: F, t44505: F, t15363: F, t15281: F, t18549: F, t1090: F, t1184: F, t15304: F, t15376: F, t15383: F, t15395: F, t18523: F, t27654: F, t3440: F, t3441: F, t44504: F, t4919: F, t52191: F, t55723: F, t18554: F, t17635: F, t11569: F, t1177: F, t1178: F, t15390: F, t18321: F, t3443: F, t3457: F, t3461: F, t3475: F, t52066: F, t52100: F, t52224: F, t52228: F, t52240: F, t52250: F, t55677: F, t11583: F, t17691: F, t15372: F, t11529: F, t6126: F, t15278: F, t15357: F, t15360: F, t18416: F, t52216: F, t52220: F, t6144: F, t8034: F, t44571: F, t6119: F, t44607: F, t15382: F, t52059: F, t15338: F, t18542: F, t15293: F, t15289: F, t15320: F, t3455: F, t52140: F, t52281: F, t52288: F, t52296: F, t15294: F, t44573: F, t44586: F, t44635: F, t44638: F, t44641: F, t52300: F, t52354: F, t52357: F, t52362: F, t52364: F, t52367: F, t64634: F, t64660: F, t64694: F, t64725: F, t64746: F, t64786: F, t64823: F, t64845: F, t64883: F, t64966: F, t19256: F, t225: F, t11606: F, t11613: F, t1190: F, t1238: F, t1252: F, t15787: F, t15794: F, t15820: F, t1761: F, t19120: F, t19214: F, t19226: F, t19232: F, t3487: F, t3593: F, t3598: F, t3599: F, t3600: F, t3630: F, t491: F, t4945: F, t498: F, t5055: F, t5089: F, t51937: F, t52386: F, t6243: F, t6244: F, t6267: F, t19211: F, t3507: F, t6238: F, t11914: F, t1244: F, t1246: F, t14997: F, t15022: F, t15023: F, t15027: F, t15239: F, t15245: F, t15430: F, t15771: F, t15777: F, t1734: F, t1751: F, t1755: F, t19138: F, t19166: F, t19190: F, t3493: F, t3604: F, t3624: F, t3625: F, t45326: F, t475: F, t5064: F, t5072: F, t53592: F, t6252: F, t6260: F, t6739: F, t3030: F, t6150: F, t3609: F, t3623: F, t5011: F, t63280: F, t64446: F, t64454: F, t64456: F, t64458: F, t64460: F, t64462: F, t64464: F, t64466: F, t64470: F, t64472: F, t64475: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t65001 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3141::<F>(t11570, t17686, t1174, t15269, t15274, t15288, t18420, t3447, t3449, t3469, t44487, t460, t4889, t4900, t4934, t6138, t64969, t64976, t64979, t64981, t64988, t64990);
        let (t65014, t65037) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3142::<F>(t15299, t4889, t15285, t12652, t14725, t17686, t44505, t15363, t1174, t15281, t18549, t1090, t1184, t15304, t15376, t15383, t15395, t18523, t27654, t3440, t3441, t3447, t44504, t460, t4919, t4934, t52191, t55723);
        let t65073 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3143::<F>(t1174, t15281, t18554, t11570, t17635, t11569, t1177, t1178, t15390, t18321, t3443, t3447, t3457, t3461, t3475, t460, t4919, t4934, t52066, t52100, t52224, t52228, t52240, t52250, t55677, t6138);
        let t65114 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3144::<F>(t11583, t17635, t11570, t17691, t15372, t4889, t11529, t1174, t6126, t11569, t15278, t15288, t15357, t15360, t18416, t3447, t3449, t3469, t3475, t460, t4919, t4934, t52216, t52220, t6144, t8034);
        let t65147 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3145::<F>(t1174, t44571, t6119, t17686, t44607, t15382, t3447, t52059, t15338, t18542, t15293, t11569, t1177, t15289, t15320, t15376, t3455, t52140, t52281, t52288, t52296, t55723);
        let t65161 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3146::<F>(t15294, t15376, t44573, t44586, t44635, t44638, t44641, t52300, t52354, t52357, t52362, t52364, t52367);
        let t65165 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3147::<F>(t64634, t64660, t64694, t64725, t64746, t64786, t64823, t64845, t64883, t64966, t65001, t65037, t65073, t65114, t65147, t65161);
        let t65206 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3148::<F>(t19256, t225, t11606, t11613, t1190, t1238, t1252, t15787, t15794, t15820, t1761, t19120, t19214, t19226, t19232, t3487, t3593, t3598, t3599, t3600, t3630, t491, t4945, t498, t5055, t5089, t51937, t52386, t6243, t6244, t6267, t65165);
        let (t65208, t65221, t65249) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3149::<F>(t19211, t225, t3507, t6238, t11914, t1244, t1246, t14997, t15022, t15023, t15027, t15239, t15245, t15430, t15771, t15777, t1734, t1751, t1755, t19138, t19166, t19190, t3493, t3604, t3624, t3625, t45326, t475, t5064, t5072, t53592, t6252, t6260, t6739);
        let (t65253, t65254, t65262, t65264, t65265, t65278) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3150::<F>(t3030, t6150, t3609, t3623, t5011, t491, t63280, t64446, t64454, t64456, t64458, t64460, t64462, t64464, t64466, t64470, t64472, t64475);
    (t65014, t65165, t65206, t65208, t65221, t65249, t65253, t65254, t65262, t65264, t65265, t65278)
}
