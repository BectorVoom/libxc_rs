//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta870 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3194;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3195;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3196;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3197;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3198;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3199;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta870(t15572: f64, t15740: f64, t11697: f64, t18382: f64, t3577: f64, t3575: f64, t62053: f64, t3624: f64, t1229: f64, t1734: f64, t375: f64, t3610: f64, t27524: f64, t607: f64, t1215: f64, t6224: f64, t1227: f64, t13969: f64, t18954: f64, t11709: f64, t15617: f64, t15702: f64, t15708: f64, t15709: f64, t15750: f64, t18236: f64, t18397: f64, t18948: f64, t19002: f64, t3247: f64, t3508: f64, t3578: f64, t45112: f64, t45119: f64, t45134: f64, t45162: f64, t5005: f64, t53220: f64, t53246: f64, t19067: f64, t1222: f64, t18297: f64, t18982: f64, t18947: f64, t3506: f64, t11719: f64, t18302: f64, t11546: f64, t1174: f64, t11814: f64, t15625: f64, t15761: f64, t18300: f64, t3440: f64, t3507: f64, t44725: f64, t44863: f64, t44906: f64, t45030: f64, t45178: f64, t4582: f64, t53267: f64, t6221: f64, t63378: f64, t63386: f64, t63394: f64, t18225: f64, t3431: f64, t18221: f64, t15522: f64, t4889: f64, t11668: f64, t11678: f64, t1177: f64, t15686: f64, t3248: f64, t3252: f64, t3494: f64, t52893: f64, t53270: f64, t53272: f64, t53274: f64, t53287: f64, t53291: f64, t5979: f64, t6225: f64, t63368: f64, t63410: f64, t64990: f64, t3545: f64, t6109: f64, t19071: f64, t3515: f64, t11728: f64, t18306: f64, t11738: f64, t19076: f64, t11692: f64, t1196: f64, t15239: f64, t15453: f64, t15507: f64, t15531: f64, t15667: f64, t1735: f64, t45224: f64, t4954: f64, t4977: f64, t52615: f64, t53360: f64, t55677: f64, t61910: f64, t6230: f64, t63402: f64, t66310: f64, t974: f64, t18940: f64, t486: f64, t15753: f64, t18375: f64, t3536: f64, t11734: f64, t1216: f64, t15594: f64, t15620: f64, t15637: f64, t19062: f64, t3243: f64, t4978: f64, t4989: f64, t53378: f64, t53387: f64, t53389: f64, t53397: f64, t53404: f64, t53410: f64, t6219: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t66360, t66363, t66372, t66374, t66378) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3194(t15572, t15740, t11697, t18382, t3577, t3575, t62053, t3624, t1229, t1734, t375, t3610);
        let (t66380, t66400) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3195(t27524, t607, t1215, t6224, t1227, t13969, t18954, t11709, t15617, t15702, t15708, t15709, t15740, t15750, t18236, t18397, t18948, t19002, t3247, t3508, t3577, t3578, t45112, t45119, t45134, t45162, t5005, t53220, t53246, t66360, t66363, t66372, t66374, t66378);
        let t66442 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3196(t1227, t13969, t19067, t1222, t18297, t18982, t18947, t3506, t11719, t18302, t11546, t1174, t11814, t15625, t15761, t18300, t3440, t3507, t44725, t44863, t44906, t45030, t45178, t4582, t5005, t53267, t6221, t63378, t63386, t63394);
        let t66480 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3197(t1174, t18225, t3431, t18221, t15522, t4889, t11668, t11678, t1177, t15686, t3248, t3252, t3440, t3494, t3577, t3578, t52893, t53270, t53272, t53274, t53287, t53291, t5979, t6225, t63368, t63410, t64990);
        let t66528 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3198(t3545, t6109, t13969, t19071, t3515, t11728, t18306, t11738, t19076, t11692, t1174, t1177, t1196, t1227, t15239, t15453, t15507, t15531, t15667, t1735, t3248, t3252, t3506, t3508, t3578, t45224, t4582, t4889, t4954, t4977, t52615, t53360, t55677, t61910, t6230, t63402, t66310, t974);
        let t66564 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3199(t18940, t486, t15753, t4889, t18375, t3536, t11668, t11728, t11734, t1216, t15507, t15594, t15620, t15637, t18300, t19062, t3243, t3506, t3515, t3577, t4582, t4978, t4989, t53378, t53387, t53389, t53397, t53404, t53410, t6219);
    (t66372, t66378, t66380, t66400, t66442, t66480, t66528, t66564)
}
