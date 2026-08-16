//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta1083 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3913;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3914;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3915;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3916;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3917;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3918;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3919;
use chunk7::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3920;
use chunk8::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3921;
use chunk9::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3922;
use chunk10::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3923;
use chunk11::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3924;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1083<F: Float>(t13790: F, t5658: F, t10022: F, t2782: F, t1882: F, t5710: F, t4086: F, t543: F, t74973: F, t1399: F, t22009: F, t3924: F, t46522: F, t47351: F, t47352: F, t47364: F, t47381: F, t47389: F, t47391: F, t49283: F, t49289: F, t49296: F, t5755: F, t6862: F, t820: F, t1398: F, t6888: F, t3999: F, t13921: F, t1883: F, t21981: F, t4004: F, t47395: F, t49268: F, t49308: F, t49313: F, t49321: F, t49325: F, t49346: F, t5675: F, t5735: F, t5745: F, t786: F, t4104: F, t213: F, t22005: F, t47403: F, t47413: F, t47417: F, t47432: F, t49354: F, t49361: F, t49378: F, t49382: F, t49386: F, t49395: F, t49399: F, t546: F, t74724: F, t23037: F, t22352: F, t2435: F, t13805: F, t14193: F, t22016: F, t4057: F, t47442: F, t47444: F, t49403: F, t49407: F, t49439: F, t73861: F, t74922: F, t74982: F, t9840: F, t74965: F, t4003: F, t5744: F, t74700: F, t4100: F, t14122: F, t21990: F, t47450: F, t47454: F, t47455: F, t49426: F, t49429: F, t49432: F, t49446: F, t49450: F, t74314: F, t22394: F, t686: F, t72: F, t9680: F, t10171: F, t13747: F, t1424: F, t14269: F, t1427: F, t4076: F, t47595: F, t47601: F, t47608: F, t47616: F, t47618: F, t47620: F, t49512: F, t49522: F, t49528: F, t5715: F, t6896: F, t74836: F, t74838: F, t74843: F, t74849: F, t74853: F, t74855: F, t74890: F, t74926: F, t74954: F, t74987: F, t75009: F, t75044: F, t75070: F, t75097: F, t75125: F, t75155: F, t75182: F, t1353: F, t5778: F, t1343: F, t1450: F, t198: F, t22279: F, t4139: F, t4140: F, t47070: F, t47072: F, t47076: F, t532: F, t5536: F, t5542: F, t73578: F, t73614: F, t73634: F, t73664: F, t73700: F, t74107: F, t74108: F, t74109: F, t74110: F, t74112: F, t74749: F, t74786: F, t74831: F, t1448: F, t13625: F, t13674: F, t21937: F, t22483: F, t3889: F, t47084: F, t49582: F, t5541: F, t74114: F, t74115: F, t74116: F, t74117: F, t74119: F, t74120: F, t21969: F, t566: F, t13600: F, t22486: F, t39989: F, t47086: F, t47088: F, t5591: F, t6836: F, t74121: F, t74122: F, t74123: F, t74124: F, t74125: F, t9599: F, t22461: F, t22470: F, t22475: F, t3829: F, t4135: F, t47092: F, t47096: F, t47098: F, t49541: F, t74126: F, t74129: F, t74131: F, t74133: F, t22466: F, t40067: F, t40072: F, t47109: F, t47116: F, t47118: F, t6816: F, t74134: F, t74135: F, t74136: F, t74137: F, t74138: F, t22479: F, t47122: F, t47124: F, t47131: F, t47138: F, t47140: F, t47142: F, t74139: F, t74141: F, t74142: F, t74143: F, t74144: F) -> (F, F, F, F, F, F) {
        let (t75198, t75209) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3913::<F>(t13790, t5658, t10022, t2782, t1882, t5710, t4086, t543, t74973, t1399, t22009, t3924, t46522, t47351, t47352, t47364, t47381, t47389, t47391, t49283, t49289, t49296, t5755, t6862, t820);
        let t75242 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3914::<F>(t1398, t2782, t4086, t543, t6888, t75198, t3999, t13921, t1883, t21981, t3924, t4004, t47395, t49268, t49308, t49313, t49321, t49325, t49346, t5675, t5735, t5745, t5755, t820);
        let t75263 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3915::<F>(t4086, t6888, t786, t4104, t213, t22005, t4004, t47403, t47413, t47417, t47432, t49354, t49361, t49378, t49382, t49386, t49395, t49399, t546, t5745, t74724);
        let t75295 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3916::<F>(t1398, t23037, t10022, t2782, t22352, t2435, t13805, t14193, t21981, t22005, t22016, t4057, t47442, t47444, t49403, t49407, t49439, t5675, t5745, t5755, t73861, t74922, t74982, t9840);
        let t75324 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3917::<F>(t2782, t4086, t543, t74965, t4003, t5744, t74982, t74700, t4100, t14122, t21990, t22005, t3924, t47450, t47454, t47455, t49426, t49429, t49432, t49446, t49450, t5735, t5745, t5755, t74314);
        let t75343 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3918::<F>(t22394, t686, t72, t9680, t10171, t13747, t1424, t14269, t1427, t4076, t47595, t47601, t47608, t47616, t47618, t47620, t49512, t49522, t49528, t5715, t6896, t74836, t74838, t74843, t74849, t74853, t74855, t74890, t74926, t74954, t74987, t75009, t75044, t75070, t75097, t75125, t75155, t75182, t75209, t75242, t75263, t75295, t75324);
        let t75357 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3919::<F>(t1353, t5778, t1343, t1450, t198, t22279, t4139, t4140, t47070, t47072, t47076, t532, t5536, t5542, t73578, t73614, t73634, t73664, t73700, t74107, t74108, t74109, t74110, t74112, t74749, t74786, t74831, t75343);
        let t75372 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3920::<F>(t1448, t5778, t13625, t13674, t21937, t22483, t3889, t4139, t47084, t49582, t5541, t5542, t74114, t74115, t74116, t74117, t74119, t74120);
        let t75386 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3921::<F>(t21969, t566, t1353, t13600, t22486, t3889, t39989, t4139, t47086, t47088, t5536, t5591, t6836, t74121, t74122, t74123, t74124, t74125, t9599);
        let t75401 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3922::<F>(t1450, t22461, t1353, t21937, t22470, t22475, t3829, t4135, t4139, t47092, t47096, t47098, t49541, t5536, t5541, t74126, t74129, t74131, t74133);
        let t75408 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3923::<F>(t22466, t3889, t40067, t40072, t4139, t47109, t47116, t47118, t6816, t74134, t74135, t74136, t74137, t74138, t9599);
        let t75412 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3924::<F>(t22479, t47122, t47124, t47131, t47138, t47140, t47142, t49541, t74139, t74141, t74142, t74143, t74144);
    (t75357, t75372, t75386, t75401, t75408, t75412)
}
