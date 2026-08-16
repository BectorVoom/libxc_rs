//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta689 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2611;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2612;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2613;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2614;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2615;
use chunk5::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2616;
use chunk6::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2617;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta689<F: Float>(t11678: F, t11697: F, t15559: F, t15713: F, t3577: F, t45124: F, t1213: F, t1735: F, t248: F, t45017: F, t10477: F, t1742: F, t11713: F, t3503: F, t1210: F, t11719: F, t13969: F, t15626: F, t11529: F, t1174: F, t4729: F, t11647: F, t1731: F, t1227: F, t15616: F, t11731: F, t11741: F, t11781: F, t45007: F, t45009: F, t45013: F, t5024: F, t14706: F, t3521: F, t11814: F, t4997: F, t15492: F, t3536: F, t15594: F, t1748: F, t3531: F, t3578: F, t44918: F, t45015: F, t45020: F, t45027: F, t45044: F, t5005: F, t52236: F, t52893: F, t11692: F, t15703: F, t11702: F, t5019: F, t3516: F, t607: F, t1734: F, t3493: F, t15458: F, t15462: F, t44951: F, t4949: F, t15615: F, t15702: F, t45049: F, t45114: F, t4582: F, t4728: F, t484: F, t48554: F, t488: F, t4978: F, t52462: F, t52897: F, t68: F, t1215: F, t5011: F, t1222: F, t15765: F, t3242: F, t3448: F, t11728: F, t15630: F, t11722: F, t1177: F, t11825: F, t15560: F, t15617: F, t1653: F, t3490: F, t3509: F, t45086: F, t45102: F, t45162: F, t45197: F, t45993: F, t46006: F, t4733: F, t4972: F, t4987: F, t5030: F, t50879: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t53064, t53067, t53079, t53081) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2611::<F>(t11678, t11697, t15559, t15713, t3577, t45124, t1213, t1735, t248, t45017, t10477, t1742);
        let (t53083, t53087, t53093, t53097, t53099) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2612::<F>(t11713, t3503, t53081, t1210, t11719, t13969, t15626, t11529, t1174, t4729, t11647, t1731);
        let t53106 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2613::<F>(t1227, t13969, t15616, t11731, t11741, t11781, t45007, t45009, t45013, t5024, t53079, t53083, t53087, t53093, t53097, t53099);
        let t53129 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2614::<F>(t1227, t14706, t248, t3521, t11814, t4997, t15492, t3536, t11781, t15594, t1748, t3531, t3578, t44918, t45015, t45020, t45027, t45044, t5005, t52236, t52893);
        let (t53135, t53142, t53144, t53149, t53155, t53158) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2615::<F>(t11692, t11697, t15703, t11702, t5019, t3516, t607, t1734, t3493, t15458, t3577, t15462);
        let t53167 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2616::<F>(t3577, t44951, t4949, t11692, t1227, t15615, t15702, t3578, t45049, t45114, t4582, t4728, t484, t48554, t488, t4978, t52462, t52897, t53135, t53142, t53144, t53149, t53155, t53158, t68);
        let (t53176, t53185, t53187, t53236) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2617::<F>(t1215, t5011, t1222, t15765, t3242, t3448, t11728, t13969, t15630, t11678, t11722, t1174, t1177, t11825, t1227, t15560, t15617, t1653, t3490, t3509, t3578, t45086, t45102, t45162, t45197, t4582, t45993, t46006, t4733, t4972, t4987, t5030, t50879);
    (t53064, t53067, t53081, t53106, t53129, t53144, t53149, t53167, t53176, t53185, t53187, t53236)
}
