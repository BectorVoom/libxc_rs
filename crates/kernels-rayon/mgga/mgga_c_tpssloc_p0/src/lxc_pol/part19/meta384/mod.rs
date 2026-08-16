//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta384 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1435;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1436;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1437;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1438;
use chunk4::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1439;
use chunk5::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1440;
use chunk6::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1441;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta384(t1128: f64, t11455: f64, t3324: f64, t3356: f64, t43748: f64, t43750: f64, t43780: f64, t43782: f64, t43784: f64, t43786: f64, t43788: f64, t43794: f64, t43798: f64, t43802: f64, t43806: f64, t43819: f64, t43727: f64, t43729: f64, t43734: f64, t43737: f64, t43740: f64, t43743: f64, t43746: f64, t43811: f64, t43816: f64, t43823: f64, t43828: f64, t423: f64, t3330: f64, t3355: f64, t427: f64, t1129: f64, t11310: f64, t11311: f64, t11345: f64, t11350: f64, t11352: f64, t11366: f64, t1137: f64, t1138: f64, t11410: f64, t11421: f64, t1156: f64, t3327: f64, t3334: f64, t3352: f64, t3359: f64, t3360: f64, t3376: f64, t3378: f64, t3403: f64, t436: f64, t43679: f64, t43692: f64, t43951: f64, t44142: f64, t44168: f64, t44202: f64, t44205: f64, t44211: f64, t44214: f64, t44220: f64, t44223: f64, t44243: f64, t44258: f64, t44274: f64, t44289: f64, t300: f64, t44115: f64, t44138: f64, t44198: f64, t1164: f64, t3396: f64, t3422: f64, t43994: f64, t43997: f64, t44000: f64, t44002: f64, t44006: f64, t44072: f64, t44080: f64, t44082: f64, t44085: f64, t44089: f64, t44092: f64, t11126: f64, t3419: f64, t11478: f64, t3411: f64, t3633: f64, t3415: f64, t3400: f64, t3375: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t44295, t44300, t44314) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1435(t1128, t11455, t3324, t3356, t43748, t43750, t43780, t43782, t43784, t43786, t43788, t43794, t43798, t43802, t43806);
        let t44327 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1436(t43819, t43727, t43729, t43734, t43737, t43740, t43743, t43746, t43811, t43816, t43823, t43828);
        let (t44342, t44348) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1437(t43748, t43750, t43780, t43782, t43784, t43786, t43788, t43794, t43798, t43802, t43806, t43819);
        let t44355 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1438(t43727, t43729, t43734, t43737, t43740, t43743, t43746, t43811, t43816, t43823, t43828, t44348);
        let (t44358, t44366) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1439(t423, t44342, t44355, t3330, t3355, t427, t1129, t11310, t11311, t11345, t11350, t11352, t11366, t1137, t1138, t11410, t11421, t1156, t3327, t3334, t3352, t3359, t3360, t3376, t3378, t3403, t436, t43679, t43692, t43951, t44142, t44168, t44202, t44205, t44211, t44214, t44220, t44223, t44243, t44258, t44274, t44289, t44295, t44300, t44314, t44327);
        let (t44369, t44372, t44373) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1440(t300, t44115, t44138, t44198, t44366, t1164, t3396, t3422, t43994, t43997, t44000, t44002, t44006, t44072, t44080, t44082, t44085, t44089, t44092);
        let (t44375, t44377, t44378, t44384, t44388, t44392) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1441(t11126, t3419, t11478, t3411, t3633, t3415, t1164, t3400, t3403, t44168, t1156, t3375);
    (t44358, t44369, t44372, t44373, t44375, t44377, t44378, t44384, t44388, t44392)
}
