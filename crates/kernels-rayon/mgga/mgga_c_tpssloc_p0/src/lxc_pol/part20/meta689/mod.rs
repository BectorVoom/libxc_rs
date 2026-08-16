//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta689 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2611;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2612;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2613;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2614;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2615;
use chunk5::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2616;
use chunk6::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2617;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta689(t11678: f64, t11697: f64, t15559: f64, t15713: f64, t3577: f64, t45124: f64, t1213: f64, t1735: f64, t248: f64, t45017: f64, t10477: f64, t1742: f64, t11713: f64, t3503: f64, t1210: f64, t11719: f64, t13969: f64, t15626: f64, t11529: f64, t1174: f64, t4729: f64, t11647: f64, t1731: f64, t1227: f64, t15616: f64, t11731: f64, t11741: f64, t11781: f64, t45007: f64, t45009: f64, t45013: f64, t5024: f64, t14706: f64, t3521: f64, t11814: f64, t4997: f64, t15492: f64, t3536: f64, t15594: f64, t1748: f64, t3531: f64, t3578: f64, t44918: f64, t45015: f64, t45020: f64, t45027: f64, t45044: f64, t5005: f64, t52236: f64, t52893: f64, t11692: f64, t15703: f64, t11702: f64, t5019: f64, t3516: f64, t607: f64, t1734: f64, t3493: f64, t15458: f64, t15462: f64, t44951: f64, t4949: f64, t15615: f64, t15702: f64, t45049: f64, t45114: f64, t4582: f64, t4728: f64, t484: f64, t48554: f64, t488: f64, t4978: f64, t52462: f64, t52897: f64, t68: f64, t1215: f64, t5011: f64, t1222: f64, t15765: f64, t3242: f64, t3448: f64, t11728: f64, t15630: f64, t11722: f64, t1177: f64, t11825: f64, t15560: f64, t15617: f64, t1653: f64, t3490: f64, t3509: f64, t45086: f64, t45102: f64, t45162: f64, t45197: f64, t45993: f64, t46006: f64, t4733: f64, t4972: f64, t4987: f64, t5030: f64, t50879: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t53064, t53067, t53079, t53081) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2611(t11678, t11697, t15559, t15713, t3577, t45124, t1213, t1735, t248, t45017, t10477, t1742);
        let (t53083, t53087, t53093, t53097, t53099) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2612(t11713, t3503, t53081, t1210, t11719, t13969, t15626, t11529, t1174, t4729, t11647, t1731);
        let t53106 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2613(t1227, t13969, t15616, t11731, t11741, t11781, t45007, t45009, t45013, t5024, t53079, t53083, t53087, t53093, t53097, t53099);
        let t53129 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2614(t1227, t14706, t248, t3521, t11814, t4997, t15492, t3536, t11781, t15594, t1748, t3531, t3578, t44918, t45015, t45020, t45027, t45044, t5005, t52236, t52893);
        let (t53135, t53142, t53144, t53149, t53155, t53158) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2615(t11692, t11697, t15703, t11702, t5019, t3516, t607, t1734, t3493, t15458, t3577, t15462);
        let t53167 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2616(t3577, t44951, t4949, t11692, t1227, t15615, t15702, t3578, t45049, t45114, t4582, t4728, t484, t48554, t488, t4978, t52462, t52897, t53135, t53142, t53144, t53149, t53155, t53158, t68);
        let (t53176, t53185, t53187, t53236) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2617(t1215, t5011, t1222, t15765, t3242, t3448, t11728, t13969, t15630, t11678, t11722, t1174, t1177, t11825, t1227, t15560, t15617, t1653, t3490, t3509, t3578, t45086, t45102, t45162, t45197, t4582, t45993, t46006, t4733, t4972, t4987, t5030, t50879);
    (t53064, t53067, t53081, t53106, t53129, t53144, t53149, t53167, t53176, t53185, t53187, t53236)
}
