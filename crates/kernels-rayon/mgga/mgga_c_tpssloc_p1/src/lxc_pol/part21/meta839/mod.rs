//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta839 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3001;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3002;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3003;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3004;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3005;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3006;
use chunk6::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3007;
use chunk7::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3008;
use chunk8::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3009;
use chunk9::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3010;
use chunk10::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3011;
use chunk11::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3012;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta839(t60391: f64, t60394: f64, t60398: f64, t60400: f64, t60429: f64, t60434: f64, t60568: f64, t60570: f64, t60946: f64, t60953: f64, t60955: f64, t60958: f64, t60961: f64, t1581: f64, t49541: f64, t60887: f64, t14473: f64, t4498: f64, t60332: f64, t942: f64, t951: f64, t959: f64, t10623: f64, t5808: f64, t17954: f64, t2907: f64, t60741: f64, t60744: f64, t60748: f64, t60750: f64, t60752: f64, t60787: f64, t60966: f64, t60970: f64, t62729: f64, t62730: f64, t62732: f64, t62733: f64, t62736: f64, t62737: f64, t17152: f64, t42972: f64, t973: f64, t10876: f64, t13969: f64, t17983: f64, t13995: f64, t14501: f64, t1020: f64, t1021: f64, t10214: f64, t10403: f64, t10408: f64, t1041: f64, t14164: f64, t14211: f64, t1539: f64, t17701: f64, t17732: f64, t18014: f64, t248: f64, t2979: f64, t3040: f64, t3070: f64, t3071: f64, t3120: f64, t360: f64, t42388: f64, t42546: f64, t42861: f64, t43343: f64, t4338: f64, t4582: f64, t4650: f64, t48612: f64, t50337: f64, t5875: f64, t59706: f64, t59711: f64, t59719: f64, t61910: f64, t10422: f64, t18020: f64, t10883: f64, t17979: f64, t17620: f64, t2960: f64, t10390: f64, t17649: f64, t17980: f64, t17984: f64, t3146: f64, t42565: f64, t43211: f64, t43307: f64, t43325: f64, t43336: f64, t43341: f64, t50343: f64, t50361: f64, t50378: f64, t50384: f64, t55723: f64, t974: f64, t5893: f64, t698: f64, t17615: f64, t3131: f64, t5866: f64, t1022: f64, t5872: f64, t10263: f64, t10413: f64, t14213: f64, t14215: f64, t14220: f64, t14228: f64, t14230: f64, t42483: f64, t43352: f64, t43354: f64, t4342: f64, t4575: f64, t49929: f64, t50324: f64, t50425: f64, t50429: f64, t5677: f64, t5894: f64, t61775: f64, t18015: f64, t1036: f64, t18010: f64, t14025: f64, t14508: f64, t13970: f64, t14511: f64, t13546: f64, t14222: f64, t1616: f64, t17156: f64, t17637: f64, t17643: f64, t3048: f64, t3088: f64, t3151: f64, t378: f64, t43382: f64, t49934: f64, t50438: f64, t50442: f64, t5885: f64, t5890: f64, t5904: f64, t61686: f64, t61717: f64, t61760: f64, t61803: f64, t61835: f64, t61876: f64, t61921: f64, t61965: f64, t62007: f64, t62042: f64, t62101: f64, t62145: f64, t62185: f64, t62225: f64, t62258: f64, t62296: f64, t62333: f64, t62362: f64, t62398: f64, t62427: f64, t62475: f64, t62512: f64, t62544: f64, t62576: f64, t62616: f64, t62648: f64, t62680: f64, t62722: f64, t5914: f64, t3166: f64, t1023: f64, t11034: f64, t11054: f64, t11059: f64, t14596: f64, t14651: f64, t18080: f64, t18083: f64, t18088: f64, t18094: f64, t18099: f64, t18104: f64, t18111: f64, t18161: f64, t3186: f64, t3188: f64, t3200: f64, t3201: f64, t43470: f64, t43562: f64, t4649: f64, t4669: f64, t4673: f64, t4689: f64, t50509: f64, t50610: f64, t5932: f64, t381: f64, t61719: f64, t1058: f64, t1060: f64, t11046: f64, t14488: f64, t14577: f64, t14630: f64, t1629: f64, t18089: f64, t18100: f64, t18112: f64, t18139: f64, t18142: f64, t18151: f64, t3180: f64, t43473: f64, t4678: f64, t5936: f64, t1057: f64, t61729: f64, t3199: f64, t61734: f64, t1061: f64, t11037: f64, t11051: f64, t11065: f64, t14574: f64, t14581: f64, t14590: f64, t14591: f64, t14608: f64, t14618: f64, t14623: f64, t14627: f64, t18131: f64, t18138: f64, t3202: f64, t43553: f64, t43554: f64, t4677: f64, t47857: f64, t5928: f64, t5933: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t62739 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3001(t60391, t60394, t60398, t60400, t60429, t60434, t60568, t60570, t60946, t60953, t60955, t60958, t60961);
        let (t62742, t62744, t62748, t62750, t62753) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3002(t1581, t49541, t60887, t14473, t4498, t60332, t942, t951, t959, t10623, t5808, t17954, t2907);
        let t62754 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3003(t60741, t60744, t60748, t60750, t60752, t60787, t60966, t60970, t62742, t62744, t62748, t62750, t62753);
        let (t62757, t62766, t62778) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3004(t62729, t62730, t62732, t62733, t62736, t62737, t62739, t62754, t17152, t42972, t973, t10876, t13969, t17983);
        let t62803 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3005(t13995, t14501, t1020, t1021, t10214, t10403, t10408, t1041, t14164, t14211, t1539, t17701, t17732, t18014, t248, t2979, t3040, t3070, t3071, t3120, t360, t42388, t42546, t42861, t43343, t4338, t4582, t4650, t48612, t50337, t5875, t59706, t59711, t59719, t61910, t62757, t62766, t62778, t973);
        let t62829 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3006(t10422, t18020, t3070, t10883, t13969, t17979, t17620, t2960, t10390, t17649, t17980, t17984, t3146, t42565, t43211, t43307, t43325, t43336, t43341, t50343, t50361, t50378, t50384, t55723, t973, t974);
        let t62871 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3007(t5893, t698, t973, t17615, t2960, t3131, t5866, t1022, t5872, t10263, t10403, t10413, t13995, t14213, t14215, t14220, t14228, t14230, t3070, t3071, t42483, t43352, t43354, t4342, t4575, t49929, t50324, t50425, t50429, t5677, t5894, t61775);
        let t62909 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3008(t10403, t10422, t18015, t1036, t18010, t14025, t14508, t13970, t14511, t10263, t10408, t13546, t14222, t14228, t1616, t17156, t17637, t17643, t3048, t3070, t3071, t3088, t3151, t378, t43382, t49934, t50438, t50442, t55723, t5885, t5890, t5904, t973, t974);
        let t62914 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3009(t61686, t61717, t61760, t61803, t61835, t61876, t61921, t61965, t62007, t62042, t62101, t62145, t62185, t62225, t62258, t62296, t62333, t62362, t62398, t62427, t62475, t62512, t62544, t62576, t62616, t62648, t62680, t62722, t62803, t62829, t62871, t62909);
        let (t62925, t62945, t62953) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3010(t3040, t5914, t3166, t5872, t1023, t11034, t11054, t11059, t14596, t14651, t18080, t18083, t18088, t18094, t18099, t18104, t18111, t18161, t3186, t3188, t3200, t3201, t43470, t43562, t4649, t4669, t4673, t4689, t50509, t50610, t5932);
        let (t62984, t62988) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3011(t381, t61719, t1058, t1060, t11034, t11046, t11059, t14488, t14577, t14630, t14651, t1629, t18089, t18100, t18112, t18139, t18142, t18151, t3166, t3180, t3186, t3188, t3200, t3201, t43473, t4678, t5866, t5932, t5936, t62945);
        let t63022 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3012(t1057, t61729, t3199, t61734, t1061, t11037, t11051, t11065, t14574, t14581, t14590, t14591, t14608, t14618, t14623, t14627, t18131, t18138, t3040, t3186, t3202, t43553, t43554, t4677, t47857, t5928, t5932, t5933, t5936);
    (t62742, t62744, t62748, t62750, t62753, t62757, t62914, t62925, t62953, t62984, t62988, t63022)
}
