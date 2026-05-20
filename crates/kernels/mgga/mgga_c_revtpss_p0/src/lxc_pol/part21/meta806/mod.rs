//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta806 (260520-c91 hierarchical CSE).
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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2934;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2935;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2936;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2937;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2938;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2939;
use chunk6::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2940;
use chunk7::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2941;
use chunk8::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2942;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta806<F: Float>(t11670: F, t15687: F, t3317: F, t127: F, t15690: F, t15689: F, t15692: F, t11916: F, t15932: F, t11922: F, t11927: F, t16026: F, t11710: F, t15964: F, t3091: F, t11683: F, t11774: F, t12131: F, t15691: F, t15693: F, t15696: F, t15963: F, t42170: F, t42172: F, t42176: F, t42190: F, t11268: F, t4820: F, t247: F, t42792: F, t4757: F, t4837: F, t15850: F, t3111: F, t43240: F, t4782: F, t2251: F, t4186: F, t10356: F, t1042: F, t1063: F, t11704: F, t11994: F, t15938: F, t15952: F, t16199: F, t1671: F, t3092: F, t3106: F, t42193: F, t42204: F, t42584: F, t4781: F, t13312: F, t606: F, t2258: F, t41296: F, t42471: F, t1469: F, t11977: F, t11859: F, t15834: F, t16076: F, t16208: F, t1675: F, t19634: F, t3117: F, t3188: F, t42195: F, t42227: F, t42230: F, t42232: F, t4806: F, t16082: F, t999: F, t3155: F, t3133: F, t4900: F, t11875: F, t15893: F, t15907: F, t15917: F, t1592: F, t15973: F, t15975: F, t16067: F, t19620: F, t19639: F, t357: F, t42240: F, t42249: F, t42251: F, t42550: F, t42621: F, t43050: F, t4583: F, t4899: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t53401, t53402, t53405, t53407, t53413, t53416) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2934::<F>(t11670, t15687, t3317, t127, t15690, t15689, t15692, t11916, t15932, t11922, t11927, t16026);
        let t53425 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2935::<F>(t11710, t15964, t3091, t11683, t11774, t12131, t15689, t15691, t15693, t15696, t15963, t42170, t42172, t42176, t42190, t53402, t53407, t53413, t53416);
        let (t53427, t53432, t53433, t53437, t53450) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2936::<F>(t11268, t4820, t247, t42792, t4757, t4837, t15850, t3111, t3091, t43240, t4782, t2251, t4186);
        let t53455 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2937::<F>(t10356, t1042, t1063, t11704, t11994, t15938, t15952, t16199, t1671, t3091, t3092, t3106, t42193, t42204, t42584, t4781, t53427, t53432, t53433, t53437, t53450);
        let t53459 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2938::<F>(t13312, t606);
        let t53464 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2939::<F>(t2258, t4186);
        let (t53473, t53474) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2940::<F>(t41296, t42471, t10356, t1469);
        let t53490 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2941::<F>(t11977, t4820, t1042, t1063, t11859, t15834, t16076, t16208, t1675, t19634, t3117, t3188, t42195, t42227, t42230, t42232, t4806, t53450, t53459, t53464, t53473, t53474);
        let (t53506, t53516, t53528) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2942::<F>(t16082, t999, t3155, t3133, t4900, t11875, t11927, t15893, t15907, t15917, t1592, t15973, t15975, t16067, t16076, t19620, t19639, t3092, t3117, t357, t42240, t42249, t42251, t42550, t42621, t43050, t4583, t4899);
    (t53401, t53405, t53425, t53450, t53455, t53459, t53464, t53474, t53490, t53506, t53516, t53528)
}
