//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta370 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1371;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1372;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1373;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1374;
use chunk4::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1375;
use chunk5::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1376;
use chunk6::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1377;
use chunk7::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1378;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta370(t3127: f64, t42340: f64, t42341: f64, t23508: f64, t3131: f64, t381: f64, t42422: f64, t3199: f64, t42741: f64, t1057: f64, t42754: f64, t10474: f64, t10482: f64, t11060: f64, t3120: f64, t11045: f64, t42332: f64, t43288: f64, t43292: f64, t1049: f64, t1058: f64, t1060: f64, t10857: f64, t11034: f64, t11037: f64, t11040: f64, t11049: f64, t11055: f64, t11059: f64, t11081: f64, t3187: f64, t3200: f64, t3201: f64, t43483: f64, t43489: f64, t43504: f64, t11013: f64, t225: f64, t10163: f64, t386: f64, t68: f64, t3175: f64, t11008: f64, t10160: f64, t10165: f64, t10167: f64, t10170: f64, t1052: f64, t1055: f64, t1061: f64, t1065: f64, t1066: f64, t11010: f64, t11024: f64, t11027: f64, t11028: f64, t11046: f64, t11048: f64, t11051: f64, t11054: f64, t11061: f64, t11067: f64, t11077: f64, t11078: f64, t11084: f64, t11085: f64, t14630: f64, t3026: f64, t3076: f64, t3166: f64, t3169: f64, t3174: f64, t3176: f64, t3180: f64, t3186: f64, t3188: f64, t3192: f64, t3193: f64, t3196: f64, t3197: f64, t3202: f64, t3204: f64, t3206: f64, t3207: f64, t384: f64, t388: f64, t42715: f64, t43082: f64, t43083: f64, t43470: f64, t43473: f64, t43512: f64, t4684: f64, t1070: f64, t11094: f64, t193: f64, t3209: f64, t3213: f64, t336: f64, t41804: f64, t41813: f64, t42276: f64, t42280: f64, t42283: f64, t42663: f64, t42665: f64, t42667: f64, t42669: f64, t42674: f64, t42678: f64, t43447: f64, t4700: f64, t3215: f64, t3216: f64, t41992: f64, t41998: f64, t42002: f64, t42005: f64, t42025: f64, t42031: f64, t42097: f64, t42105: f64, t42682: f64, t42686: f64, t42688: f64, t42145: f64, t42148: f64, t42233: f64, t42235: f64, t42238: f64, t42241: f64, t42692: f64, t42697: f64, t42699: f64, t42701: f64, t42704: f64, t42708: f64, t42712: f64, t25: f64, t265: f64, t394: f64, t41606: f64, t42274: f64, t10150: f64, t1074: f64, t11105: f64, t2249: f64, t2250: f64, t2756: f64, t3220: f64, t39109: f64, t39110: f64, t396: f64, t40: f64, t606: f64, t607: f64, t873: f64, t9257: f64, t9258: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64, t11126: f64, t3423: f64, t11286: f64, t3411: f64, t11629: f64, t11399: f64, t1164: f64, t3400: f64, t4883: f64, t3377: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t43515, t43516, t43525, t43536, t43542, t43553) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1371(t3127, t42340, t42341, t23508, t3131, t381, t42422, t3199, t42741, t1057, t42754, t10474);
        let t43584 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1372(t10482, t23508, t11060, t3120, t11045, t42332, t42340, t42341, t43288, t43292, t1049, t1058, t1060, t10857, t11034, t11037, t11040, t11049, t11055, t11059, t11081, t3187, t3200, t3201, t43483, t43489, t43504, t43525, t43553);
        let t43622 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1373(t11013, t225, t10163, t386, t68, t3175, t11008, t10160, t10165, t10167, t10170, t1052, t1055, t1058, t1060, t1061, t1065, t1066, t11010, t11024, t11027, t11028, t11034, t11046, t11048, t11051, t11054, t11061, t11067, t11077, t11078, t11084, t11085, t14630, t3026, t3076, t3120, t3166, t3169, t3174, t3176, t3180, t3186, t3188, t3192, t3193, t3196, t3197, t3200, t3202, t3204, t3206, t3207, t381, t384, t388, t42715, t43082, t43083, t43470, t43473, t43483, t43504, t43512, t43515, t43516, t43525, t43536, t43542, t43584, t4684);
        let t43627 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1374(t1070, t11094, t193, t3209, t3213, t336, t41804, t41813, t42276, t42280, t42283, t42663, t42665, t42667, t42669, t42674, t42678, t43447, t43622, t4700);
        let t43641 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1375(t3209, t3213, t3215, t193, t3216, t336, t41992, t41998, t42002, t42005, t42025, t42031, t42097, t42105, t42682, t42686, t42688);
        let t43642 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1376(t42145, t42148, t42233, t42235, t42238, t42241, t42692, t42697, t42699, t42701, t42704, t42708, t42712);
        let t43657 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1377(t25, t265, t394, t41606, t42274, t43627, t43641, t43642, t10150, t1074, t11105, t2249, t2250, t2756, t3220, t39109, t39110, t396, t40, t606, t607, t873, t9257, t9258, dens_threshold, rho0, zeta_threshold);
        let (t43670, t43672, t43674, t43678, t43679) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1378(t11126, t3423, t11286, t3411, t11629, t11399, t1164, t3400, t4883, t3377);
    (t43657, t43670, t43672, t43674, t43678, t43679)
}
