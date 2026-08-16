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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

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
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1083(t13790: f64, t5658: f64, t10022: f64, t2782: f64, t1882: f64, t5710: f64, t4086: f64, t543: f64, t74973: f64, t1399: f64, t22009: f64, t3924: f64, t46522: f64, t47351: f64, t47352: f64, t47364: f64, t47381: f64, t47389: f64, t47391: f64, t49283: f64, t49289: f64, t49296: f64, t5755: f64, t6862: f64, t820: f64, t1398: f64, t6888: f64, t3999: f64, t13921: f64, t1883: f64, t21981: f64, t4004: f64, t47395: f64, t49268: f64, t49308: f64, t49313: f64, t49321: f64, t49325: f64, t49346: f64, t5675: f64, t5735: f64, t5745: f64, t786: f64, t4104: f64, t213: f64, t22005: f64, t47403: f64, t47413: f64, t47417: f64, t47432: f64, t49354: f64, t49361: f64, t49378: f64, t49382: f64, t49386: f64, t49395: f64, t49399: f64, t546: f64, t74724: f64, t23037: f64, t22352: f64, t2435: f64, t13805: f64, t14193: f64, t22016: f64, t4057: f64, t47442: f64, t47444: f64, t49403: f64, t49407: f64, t49439: f64, t73861: f64, t74922: f64, t74982: f64, t9840: f64, t74965: f64, t4003: f64, t5744: f64, t74700: f64, t4100: f64, t14122: f64, t21990: f64, t47450: f64, t47454: f64, t47455: f64, t49426: f64, t49429: f64, t49432: f64, t49446: f64, t49450: f64, t74314: f64, t22394: f64, t686: f64, t72: f64, t9680: f64, t10171: f64, t13747: f64, t1424: f64, t14269: f64, t1427: f64, t4076: f64, t47595: f64, t47601: f64, t47608: f64, t47616: f64, t47618: f64, t47620: f64, t49512: f64, t49522: f64, t49528: f64, t5715: f64, t6896: f64, t74836: f64, t74838: f64, t74843: f64, t74849: f64, t74853: f64, t74855: f64, t74890: f64, t74926: f64, t74954: f64, t74987: f64, t75009: f64, t75044: f64, t75070: f64, t75097: f64, t75125: f64, t75155: f64, t75182: f64, t1353: f64, t5778: f64, t1343: f64, t1450: f64, t198: f64, t22279: f64, t4139: f64, t4140: f64, t47070: f64, t47072: f64, t47076: f64, t532: f64, t5536: f64, t5542: f64, t73578: f64, t73614: f64, t73634: f64, t73664: f64, t73700: f64, t74107: f64, t74108: f64, t74109: f64, t74110: f64, t74112: f64, t74749: f64, t74786: f64, t74831: f64, t1448: f64, t13625: f64, t13674: f64, t21937: f64, t22483: f64, t3889: f64, t47084: f64, t49582: f64, t5541: f64, t74114: f64, t74115: f64, t74116: f64, t74117: f64, t74119: f64, t74120: f64, t21969: f64, t566: f64, t13600: f64, t22486: f64, t39989: f64, t47086: f64, t47088: f64, t5591: f64, t6836: f64, t74121: f64, t74122: f64, t74123: f64, t74124: f64, t74125: f64, t9599: f64, t22461: f64, t22470: f64, t22475: f64, t3829: f64, t4135: f64, t47092: f64, t47096: f64, t47098: f64, t49541: f64, t74126: f64, t74129: f64, t74131: f64, t74133: f64, t22466: f64, t40067: f64, t40072: f64, t47109: f64, t47116: f64, t47118: f64, t6816: f64, t74134: f64, t74135: f64, t74136: f64, t74137: f64, t74138: f64, t22479: f64, t47122: f64, t47124: f64, t47131: f64, t47138: f64, t47140: f64, t47142: f64, t74139: f64, t74141: f64, t74142: f64, t74143: f64, t74144: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t75198, t75209) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3913(t13790, t5658, t10022, t2782, t1882, t5710, t4086, t543, t74973, t1399, t22009, t3924, t46522, t47351, t47352, t47364, t47381, t47389, t47391, t49283, t49289, t49296, t5755, t6862, t820);
        let t75242 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3914(t1398, t2782, t4086, t543, t6888, t75198, t3999, t13921, t1883, t21981, t3924, t4004, t47395, t49268, t49308, t49313, t49321, t49325, t49346, t5675, t5735, t5745, t5755, t820);
        let t75263 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3915(t4086, t6888, t786, t4104, t213, t22005, t4004, t47403, t47413, t47417, t47432, t49354, t49361, t49378, t49382, t49386, t49395, t49399, t546, t5745, t74724);
        let t75295 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3916(t1398, t23037, t10022, t2782, t22352, t2435, t13805, t14193, t21981, t22005, t22016, t4057, t47442, t47444, t49403, t49407, t49439, t5675, t5745, t5755, t73861, t74922, t74982, t9840);
        let t75324 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3917(t2782, t4086, t543, t74965, t4003, t5744, t74982, t74700, t4100, t14122, t21990, t22005, t3924, t47450, t47454, t47455, t49426, t49429, t49432, t49446, t49450, t5735, t5745, t5755, t74314);
        let t75343 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3918(t22394, t686, t72, t9680, t10171, t13747, t1424, t14269, t1427, t4076, t47595, t47601, t47608, t47616, t47618, t47620, t49512, t49522, t49528, t5715, t6896, t74836, t74838, t74843, t74849, t74853, t74855, t74890, t74926, t74954, t74987, t75009, t75044, t75070, t75097, t75125, t75155, t75182, t75209, t75242, t75263, t75295, t75324);
        let t75357 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3919(t1353, t5778, t1343, t1450, t198, t22279, t4139, t4140, t47070, t47072, t47076, t532, t5536, t5542, t73578, t73614, t73634, t73664, t73700, t74107, t74108, t74109, t74110, t74112, t74749, t74786, t74831, t75343);
        let t75372 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3920(t1448, t5778, t13625, t13674, t21937, t22483, t3889, t4139, t47084, t49582, t5541, t5542, t74114, t74115, t74116, t74117, t74119, t74120);
        let t75386 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3921(t21969, t566, t1353, t13600, t22486, t3889, t39989, t4139, t47086, t47088, t5536, t5591, t6836, t74121, t74122, t74123, t74124, t74125, t9599);
        let t75401 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3922(t1450, t22461, t1353, t21937, t22470, t22475, t3829, t4135, t4139, t47092, t47096, t47098, t49541, t5536, t5541, t74126, t74129, t74131, t74133);
        let t75408 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3923(t22466, t3889, t40067, t40072, t4139, t47109, t47116, t47118, t6816, t74134, t74135, t74136, t74137, t74138, t9599);
        let t75412 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3924(t22479, t47122, t47124, t47131, t47138, t47140, t47142, t49541, t74139, t74141, t74142, t74143, t74144);
    (t75357, t75372, t75386, t75401, t75408, t75412)
}
