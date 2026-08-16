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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

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
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta806(t11670: f64, t15687: f64, t3317: f64, t127: f64, t15690: f64, t15689: f64, t15692: f64, t11916: f64, t15932: f64, t11922: f64, t11927: f64, t16026: f64, t11710: f64, t15964: f64, t3091: f64, t11683: f64, t11774: f64, t12131: f64, t15691: f64, t15693: f64, t15696: f64, t15963: f64, t42170: f64, t42172: f64, t42176: f64, t42190: f64, t11268: f64, t4820: f64, t247: f64, t42792: f64, t4757: f64, t4837: f64, t15850: f64, t3111: f64, t43240: f64, t4782: f64, t2251: f64, t4186: f64, t10356: f64, t1042: f64, t1063: f64, t11704: f64, t11994: f64, t15938: f64, t15952: f64, t16199: f64, t1671: f64, t3092: f64, t3106: f64, t42193: f64, t42204: f64, t42584: f64, t4781: f64, t13312: f64, t606: f64, t2258: f64, t41296: f64, t42471: f64, t1469: f64, t11977: f64, t11859: f64, t15834: f64, t16076: f64, t16208: f64, t1675: f64, t19634: f64, t3117: f64, t3188: f64, t42195: f64, t42227: f64, t42230: f64, t42232: f64, t4806: f64, t16082: f64, t999: f64, t3155: f64, t3133: f64, t4900: f64, t11875: f64, t15893: f64, t15907: f64, t15917: f64, t1592: f64, t15973: f64, t15975: f64, t16067: f64, t19620: f64, t19639: f64, t357: f64, t42240: f64, t42249: f64, t42251: f64, t42550: f64, t42621: f64, t43050: f64, t4583: f64, t4899: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t53401, t53402, t53405, t53407, t53413, t53416) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2934(t11670, t15687, t3317, t127, t15690, t15689, t15692, t11916, t15932, t11922, t11927, t16026);
        let t53425 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2935(t11710, t15964, t3091, t11683, t11774, t12131, t15689, t15691, t15693, t15696, t15963, t42170, t42172, t42176, t42190, t53402, t53407, t53413, t53416);
        let (t53427, t53432, t53433, t53437, t53450) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2936(t11268, t4820, t247, t42792, t4757, t4837, t15850, t3111, t3091, t43240, t4782, t2251, t4186);
        let t53455 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2937(t10356, t1042, t1063, t11704, t11994, t15938, t15952, t16199, t1671, t3091, t3092, t3106, t42193, t42204, t42584, t4781, t53427, t53432, t53433, t53437, t53450);
        let t53459 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2938(t13312, t606);
        let t53464 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2939(t2258, t4186);
        let (t53473, t53474) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2940(t41296, t42471, t10356, t1469);
        let t53490 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2941(t11977, t4820, t1042, t1063, t11859, t15834, t16076, t16208, t1675, t19634, t3117, t3188, t42195, t42227, t42230, t42232, t4806, t53450, t53459, t53464, t53473, t53474);
        let (t53506, t53516, t53528) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2942(t16082, t999, t3155, t3133, t4900, t11875, t11927, t15893, t15907, t15917, t1592, t15973, t15975, t16067, t16076, t19620, t19639, t3092, t3117, t357, t42240, t42249, t42251, t42550, t42621, t43050, t4583, t4899);
    (t53401, t53405, t53425, t53450, t53455, t53459, t53464, t53474, t53490, t53506, t53516, t53528)
}
