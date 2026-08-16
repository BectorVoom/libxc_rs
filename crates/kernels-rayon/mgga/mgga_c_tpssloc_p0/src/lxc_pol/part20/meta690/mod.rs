//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta690 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2618;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2619;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2620;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2621;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2622;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta690(t11718: f64, t52835: f64, t11797: f64, t5024: f64, t11147: f64, t15394: f64, t11665: f64, t11724: f64, t11774: f64, t15455: f64, t15459: f64, t15463: f64, t3447: f64, t3490: f64, t45108: f64, t45112: f64, t45126: f64, t45148: f64, t45971: f64, t5005: f64, t1174: f64, t5045: f64, t698: f64, t3540: f64, t4966: f64, t11647: f64, t1744: f64, t11825: f64, t45167: f64, t45169: f64, t45171: f64, t45178: f64, t45181: f64, t45184: f64, t4974: f64, t11697: f64, t15469: f64, t3577: f64, t11801: f64, t3247: f64, t475: f64, t15032: f64, t3576: f64, t11713: f64, t11716: f64, t53081: f64, t11148: f64, t11674: f64, t11678: f64, t11766: f64, t11855: f64, t1216: f64, t14706: f64, t15470: f64, t15661: f64, t15663: f64, t15740: f64, t1735: f64, t18946: f64, t3578: f64, t3580: f64, t45128: f64, t45162: f64, t45211: f64, t4889: f64, t5019: f64, t11786: f64, t3509: f64, t607: f64, t3032: f64, t52434: f64, t3505: f64, t1090: f64, t1196: f64, t15525: f64, t15591: f64, t3252: f64, t3496: f64, t3511: f64, t45222: f64, t45224: f64, t45227: f64, t45872: f64, t4728: f64, t5002: f64, t5012: f64, t974: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t53258 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2618(t11718, t52835, t11797, t5024, t11147, t15394, t11665, t11724, t11774, t15455, t15459, t15463, t3447, t3490, t45108, t45112, t45126, t45148, t45971, t5005);
        let t53276 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2619(t11797, t5005, t1174, t5045, t698, t3540, t4966, t11647, t1744, t11825, t45167, t45169, t45171, t45178, t45181, t45184, t4974);
        let (t53287, t53291, t53298, t53322, t53336) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2620(t11697, t15469, t3577, t11801, t5005, t3247, t475, t15032, t3576, t11713, t11716, t53081);
        let t53345 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2621(t11148, t11665, t11674, t11678, t11724, t11766, t11855, t1216, t14706, t15470, t15661, t15663, t15740, t1735, t18946, t3577, t3578, t3580, t45128, t45162, t45211, t4889, t5019, t53322, t53336);
        let (t53366, t53371, t53377) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2622(t11786, t5024, t3509, t607, t3032, t52434, t3505, t1090, t11678, t1174, t11855, t1196, t15525, t15591, t3252, t3496, t3511, t3577, t3578, t45222, t45224, t45227, t45872, t4728, t5002, t5012, t974);
    (t53258, t53276, t53287, t53291, t53298, t53345, t53366, t53371, t53377)
}
