//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta384 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1435;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1436;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1437;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1438;
use chunk4::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1439;
use chunk5::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1440;
use chunk6::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1441;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta384<F: Float>(t1128: F, t11455: F, t3324: F, t3356: F, t43748: F, t43750: F, t43780: F, t43782: F, t43784: F, t43786: F, t43788: F, t43794: F, t43798: F, t43802: F, t43806: F, t43819: F, t43727: F, t43729: F, t43734: F, t43737: F, t43740: F, t43743: F, t43746: F, t43811: F, t43816: F, t43823: F, t43828: F, t423: F, t3330: F, t3355: F, t427: F, t1129: F, t11310: F, t11311: F, t11345: F, t11350: F, t11352: F, t11366: F, t1137: F, t1138: F, t11410: F, t11421: F, t1156: F, t3327: F, t3334: F, t3352: F, t3359: F, t3360: F, t3376: F, t3378: F, t3403: F, t436: F, t43679: F, t43692: F, t43951: F, t44142: F, t44168: F, t44202: F, t44205: F, t44211: F, t44214: F, t44220: F, t44223: F, t44243: F, t44258: F, t44274: F, t44289: F, t300: F, t44115: F, t44138: F, t44198: F, t1164: F, t3396: F, t3422: F, t43994: F, t43997: F, t44000: F, t44002: F, t44006: F, t44072: F, t44080: F, t44082: F, t44085: F, t44089: F, t44092: F, t11126: F, t3419: F, t11478: F, t3411: F, t3633: F, t3415: F, t3400: F, t3375: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t44295, t44300, t44314) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1435::<F>(t1128, t11455, t3324, t3356, t43748, t43750, t43780, t43782, t43784, t43786, t43788, t43794, t43798, t43802, t43806);
        let t44327 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1436::<F>(t43819, t43727, t43729, t43734, t43737, t43740, t43743, t43746, t43811, t43816, t43823, t43828);
        let (t44342, t44348) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1437::<F>(t43748, t43750, t43780, t43782, t43784, t43786, t43788, t43794, t43798, t43802, t43806, t43819);
        let t44355 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1438::<F>(t43727, t43729, t43734, t43737, t43740, t43743, t43746, t43811, t43816, t43823, t43828, t44348);
        let (t44358, t44366) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1439::<F>(t423, t44342, t44355, t3330, t3355, t427, t1129, t11310, t11311, t11345, t11350, t11352, t11366, t1137, t1138, t11410, t11421, t1156, t3327, t3334, t3352, t3359, t3360, t3376, t3378, t3403, t436, t43679, t43692, t43951, t44142, t44168, t44202, t44205, t44211, t44214, t44220, t44223, t44243, t44258, t44274, t44289, t44295, t44300, t44314, t44327);
        let (t44369, t44372, t44373) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1440::<F>(t300, t44115, t44138, t44198, t44366, t1164, t3396, t3422, t43994, t43997, t44000, t44002, t44006, t44072, t44080, t44082, t44085, t44089, t44092);
        let (t44375, t44377, t44378, t44384, t44388, t44392) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1441::<F>(t11126, t3419, t11478, t3411, t3633, t3415, t1164, t3400, t3403, t44168, t1156, t3375);
    (t44358, t44369, t44372, t44373, t44375, t44377, t44378, t44384, t44388, t44392)
}
