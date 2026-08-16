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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1371;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1372;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1373;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1374;
use chunk4::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1375;
use chunk5::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1376;
use chunk6::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1377;
use chunk7::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1378;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta370<F: Float>(t3127: F, t42340: F, t42341: F, t23508: F, t3131: F, t381: F, t42422: F, t3199: F, t42741: F, t1057: F, t42754: F, t10474: F, t10482: F, t11060: F, t3120: F, t11045: F, t42332: F, t43288: F, t43292: F, t1049: F, t1058: F, t1060: F, t10857: F, t11034: F, t11037: F, t11040: F, t11049: F, t11055: F, t11059: F, t11081: F, t3187: F, t3200: F, t3201: F, t43483: F, t43489: F, t43504: F, t11013: F, t225: F, t10163: F, t386: F, t68: F, t3175: F, t11008: F, t10160: F, t10165: F, t10167: F, t10170: F, t1052: F, t1055: F, t1061: F, t1065: F, t1066: F, t11010: F, t11024: F, t11027: F, t11028: F, t11046: F, t11048: F, t11051: F, t11054: F, t11061: F, t11067: F, t11077: F, t11078: F, t11084: F, t11085: F, t14630: F, t3026: F, t3076: F, t3166: F, t3169: F, t3174: F, t3176: F, t3180: F, t3186: F, t3188: F, t3192: F, t3193: F, t3196: F, t3197: F, t3202: F, t3204: F, t3206: F, t3207: F, t384: F, t388: F, t42715: F, t43082: F, t43083: F, t43470: F, t43473: F, t43512: F, t4684: F, t1070: F, t11094: F, t193: F, t3209: F, t3213: F, t336: F, t41804: F, t41813: F, t42276: F, t42280: F, t42283: F, t42663: F, t42665: F, t42667: F, t42669: F, t42674: F, t42678: F, t43447: F, t4700: F, t3215: F, t3216: F, t41992: F, t41998: F, t42002: F, t42005: F, t42025: F, t42031: F, t42097: F, t42105: F, t42682: F, t42686: F, t42688: F, t42145: F, t42148: F, t42233: F, t42235: F, t42238: F, t42241: F, t42692: F, t42697: F, t42699: F, t42701: F, t42704: F, t42708: F, t42712: F, t25: F, t265: F, t394: F, t41606: F, t42274: F, t10150: F, t1074: F, t11105: F, t2249: F, t2250: F, t2756: F, t3220: F, t39109: F, t39110: F, t396: F, t40: F, t606: F, t607: F, t873: F, t9257: F, t9258: F, dens_threshold: F, rho0: F, zeta_threshold: F, t11126: F, t3423: F, t11286: F, t3411: F, t11629: F, t11399: F, t1164: F, t3400: F, t4883: F, t3377: F) -> (F, F, F, F, F, F) {
        let (t43515, t43516, t43525, t43536, t43542, t43553) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1371::<F>(t3127, t42340, t42341, t23508, t3131, t381, t42422, t3199, t42741, t1057, t42754, t10474);
        let t43584 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1372::<F>(t10482, t23508, t11060, t3120, t11045, t42332, t42340, t42341, t43288, t43292, t1049, t1058, t1060, t10857, t11034, t11037, t11040, t11049, t11055, t11059, t11081, t3187, t3200, t3201, t43483, t43489, t43504, t43525, t43553);
        let t43622 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1373::<F>(t11013, t225, t10163, t386, t68, t3175, t11008, t10160, t10165, t10167, t10170, t1052, t1055, t1058, t1060, t1061, t1065, t1066, t11010, t11024, t11027, t11028, t11034, t11046, t11048, t11051, t11054, t11061, t11067, t11077, t11078, t11084, t11085, t14630, t3026, t3076, t3120, t3166, t3169, t3174, t3176, t3180, t3186, t3188, t3192, t3193, t3196, t3197, t3200, t3202, t3204, t3206, t3207, t381, t384, t388, t42715, t43082, t43083, t43470, t43473, t43483, t43504, t43512, t43515, t43516, t43525, t43536, t43542, t43584, t4684);
        let t43627 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1374::<F>(t1070, t11094, t193, t3209, t3213, t336, t41804, t41813, t42276, t42280, t42283, t42663, t42665, t42667, t42669, t42674, t42678, t43447, t43622, t4700);
        let t43641 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1375::<F>(t3209, t3213, t3215, t193, t3216, t336, t41992, t41998, t42002, t42005, t42025, t42031, t42097, t42105, t42682, t42686, t42688);
        let t43642 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1376::<F>(t42145, t42148, t42233, t42235, t42238, t42241, t42692, t42697, t42699, t42701, t42704, t42708, t42712);
        let t43657 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1377::<F>(t25, t265, t394, t41606, t42274, t43627, t43641, t43642, t10150, t1074, t11105, t2249, t2250, t2756, t3220, t39109, t39110, t396, t40, t606, t607, t873, t9257, t9258, dens_threshold, rho0, zeta_threshold);
        let (t43670, t43672, t43674, t43678, t43679) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1378::<F>(t11126, t3423, t11286, t3411, t11629, t11399, t1164, t3400, t4883, t3377);
    (t43657, t43670, t43672, t43674, t43678, t43679)
}
