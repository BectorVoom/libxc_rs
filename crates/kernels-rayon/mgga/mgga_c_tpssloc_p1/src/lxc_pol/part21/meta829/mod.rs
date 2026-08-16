//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta829 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2922;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2923;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2924;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2925;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta829(t14344: f64, t4488: f64, t959: f64, t11094: f64, t5946: f64, t1068: f64, t3213: f64, t4700: f64, t60842: f64, t60844: f64, t60847: f64, t60850: f64, t60852: f64, t60855: f64, t60857: f64, t60860: f64, t60862: f64, t60864: f64, t60866: f64, t60867: f64, t4696: f64, t13732: f64, t4483: f64, t4471: f64, t950: f64, t14369: f64, t49513: f64, t4475: f64, t49532: f64, t4496: f64, t48883: f64, t10523: f64, t2933: f64, t5790: f64, t14662: f64, t193: f64, t3216: f64, t336: f64, t4701: f64, t59891: f64, t59958: f64, t59961: f64, t59966: f64, t59968: f64, t59970: f64, t59972: f64, t17934: f64, t2944: f64, t10623: f64, t5804: f64, t59981: f64, t60006: f64, t60008: f64, t60010: f64, t60016: f64, t60021: f64, t60023: f64, t60025: f64, t60027: f64, t60029: f64, t60033: f64, t60035: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t60873, t60878) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2922(t14344, t4488, t959, t11094, t5946, t1068, t3213, t4700, t60842, t60844, t60847, t60850, t60852, t60855, t60857, t60860, t60862, t60864, t60866, t60867);
        let (t60880, t60886, t60887, t60890, t60893, t60899) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2923(t4696, t13732, t4483, t4471, t950, t14369, t49513, t4475, t49532, t4496, t48883, t959);
        let (t60903, t60904) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2924(t10523, t2933, t5790, t959, t14662, t193, t3216, t336, t4700, t4701, t59891, t59958, t59961, t59966, t59968, t59970, t59972, t60880, t60886, t60890, t60893, t60899);
        let (t60906, t60908, t60909) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2925(t17934, t2944, t10623, t5804, t59981, t60006, t60008, t60010, t60016, t60021, t60023, t60025, t60027, t60029, t60033, t60035);
    (t60873, t60878, t60886, t60887, t60890, t60893, t60899, t60903, t60904, t60906, t60908, t60909)
}
