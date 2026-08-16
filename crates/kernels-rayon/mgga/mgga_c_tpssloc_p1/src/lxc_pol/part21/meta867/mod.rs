//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta867 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3164;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3165;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3166;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3167;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3168;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3169;
use chunk6::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3170;
use chunk7::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3171;
use chunk8::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3172;
use chunk9::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3173;
use chunk10::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3174;
use chunk11::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3175;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta867(t3508: f64, t6218: f64, t1215: f64, t11721: f64, t6224: f64, t15594: f64, t4993: f64, t11692: f64, t11697: f64, t18396: f64, t18400: f64, t3577: f64, t1653: f64, t3507: f64, t11678: f64, t19001: f64, t11825: f64, t14726: f64, t15659: f64, t15661: f64, t15702: f64, t1735: f64, t18395: f64, t19083: f64, t19101: f64, t3490: f64, t3493: f64, t3578: f64, t3587: f64, t45114: f64, t45128: f64, t45197: f64, t52704: f64, t53149: f64, t6207: f64, t11818: f64, t1213: f64, t248: f64, t6219: f64, t3036: f64, t6163: f64, t3500: f64, t3503: f64, t1210: f64, t15734: f64, t5005: f64, t19047: f64, t3572: f64, t3506: f64, t6225: f64, t1174: f64, t1214: f64, t1227: f64, t1230: f64, t15672: f64, t15761: f64, t1737: f64, t19026: f64, t19051: f64, t3440: f64, t3496: f64, t3511: f64, t3515: f64, t3518: f64, t475: f64, t4889: f64, t5024: f64, t52568: f64, t6211: f64, t63311: f64, t63353: f64, t65264: f64, t11539: f64, t18211: f64, t3540: f64, t6170: f64, t19015: f64, t45124: f64, t6158: f64, t15730: f64, t5002: f64, t1226: f64, t18573: f64, t11546: f64, t1232: f64, t14744: f64, t14753: f64, t15569: f64, t15710: f64, t15764: f64, t1743: f64, t3447: f64, t3566: f64, t45119: f64, t488: f64, t52696: f64, t52995: f64, t53187: f64, t55716: f64, t6164: f64, t63372: f64, t18392: f64, t18241: f64, t3521: f64, t19040: f64, t6230: f64, t15578: f64, t11789: f64, t5979: f64, t3523: f64, t19080: f64, t1177: f64, t15581: f64, t15584: f64, t15587: f64, t6203: f64, t63406: f64, t65330: f64, t11709: f64, t18356: f64, t18975: f64, t6165: f64, t19033: f64, t11734: f64, t19095: f64, t15486: f64, t1222: f64, t18574: f64, t15527: f64, t1748: f64, t3527: f64, t3531: f64, t5019: f64, t53487: f64, t63390: f64, t5975: f64, t18321: f64, t3548: f64, t15437: f64, t15502: f64, t15506: f64, t4965: f64, t5023: f64, t15498: f64, t44811: f64, t4974: f64, t52575: f64, t52580: f64, t52583: f64, t52586: f64, t52599: f64, t11668: f64, t14706: f64, t15470: f64, t15474: f64, t15560: f64, t15564: f64, t15615: f64, t15681: f64, t15740: f64, t3516: f64, t4582: f64, t4972: f64, t5030: f64, t50992: f64, t51002: f64, t52609: f64, t52619: f64, t52766: f64, t52879: f64, t55662: f64, t5971: f64, t61855: f64, t61910: f64, t62044: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t65464, t65469, t65474, t65479, t65482, t65485) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3164(t3508, t6218, t1215, t11721, t6224, t15594, t4993, t11692, t11697, t18396, t18400, t3577);
        let t65518 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3165(t1653, t3507, t11678, t11697, t19001, t11692, t11825, t14726, t15659, t15661, t15702, t1735, t18395, t19083, t19101, t3490, t3493, t3577, t3578, t3587, t45114, t45128, t45197, t52704, t53149, t6207, t65464, t65469, t65474, t65479, t65482, t65485);
        let (t65528, t65541, t65545, t65552, t65554) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3166(t11818, t1213, t248, t6219, t3036, t6163, t3500, t3503, t1210, t15734, t5005, t19047, t3572);
        let t65565 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3167(t11818, t248, t3506, t6225, t1174, t11825, t1214, t1227, t1230, t15672, t15761, t1737, t19026, t19051, t3440, t3496, t3511, t3515, t3518, t3587, t475, t4889, t5024, t52568, t6211, t63311, t63353, t65264, t65528, t65541, t65545, t65552, t65554);
        let (t65567, t65581, t65598, t65600, t65605, t65607) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3168(t11539, t1174, t18211, t3540, t6170, t19015, t3577, t45124, t6158, t15730, t5002, t1226, t18573);
        let t65610 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3169(t11546, t1174, t1232, t14744, t14753, t15569, t15710, t15764, t1735, t1743, t18395, t3447, t3566, t3577, t3578, t45119, t488, t52696, t52995, t53187, t55716, t6164, t63372, t65567, t65581, t65598, t65600, t65605, t65607);
        let (t65613, t65617, t65619, t65628, t65632) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3170(t18392, t3490, t1227, t18241, t248, t3521, t19040, t15734, t5024, t11818, t3515, t6230);
        let t65653 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3171(t15578, t4889, t11789, t1227, t248, t5979, t19051, t3523, t19080, t3572, t1174, t1177, t11825, t1213, t1214, t15581, t15584, t15587, t475, t6203, t63406, t65330, t65613, t65617, t65619, t65628, t65632);
        let (t65660, t65662, t65664, t65668, t65670, t65672, t65674) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3172(t11709, t18356, t18975, t3490, t3540, t6165, t19083, t3523, t19026, t3572, t19033, t11734, t19095);
        let t65685 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3173(t15486, t5005, t1222, t18574, t1174, t15527, t1748, t19033, t3440, t3527, t3531, t3587, t5019, t53487, t63390, t65660, t65662, t65664, t65668, t65670, t65672, t65674);
        let t65716 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3174(t11789, t1227, t248, t5975, t18321, t3548, t15437, t15502, t15506, t4965, t5023, t1232, t15498, t15594, t19083, t3511, t3518, t3527, t3531, t44811, t4974, t52575, t52580, t52583, t52586, t52599);
        let t65764 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3175(t11668, t11692, t1214, t1227, t14706, t15470, t15474, t15560, t15564, t15594, t15615, t15681, t15740, t1735, t248, t3506, t3508, t3516, t3577, t3578, t4582, t4889, t4972, t5030, t50992, t51002, t52609, t52619, t52766, t52879, t55662, t5971, t61855, t61910, t62044, t65264);
    (t65518, t65565, t65610, t65653, t65685, t65716, t65764)
}
