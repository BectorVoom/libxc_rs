//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta829 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2922;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2923;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2924;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2925;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta829<F: Float>(t14344: F, t4488: F, t959: F, t11094: F, t5946: F, t1068: F, t3213: F, t4700: F, t60842: F, t60844: F, t60847: F, t60850: F, t60852: F, t60855: F, t60857: F, t60860: F, t60862: F, t60864: F, t60866: F, t60867: F, t4696: F, t13732: F, t4483: F, t4471: F, t950: F, t14369: F, t49513: F, t4475: F, t49532: F, t4496: F, t48883: F, t10523: F, t2933: F, t5790: F, t14662: F, t193: F, t3216: F, t336: F, t4701: F, t59891: F, t59958: F, t59961: F, t59966: F, t59968: F, t59970: F, t59972: F, t17934: F, t2944: F, t10623: F, t5804: F, t59981: F, t60006: F, t60008: F, t60010: F, t60016: F, t60021: F, t60023: F, t60025: F, t60027: F, t60029: F, t60033: F, t60035: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t60873, t60878) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2922::<F>(t14344, t4488, t959, t11094, t5946, t1068, t3213, t4700, t60842, t60844, t60847, t60850, t60852, t60855, t60857, t60860, t60862, t60864, t60866, t60867);
        let (t60880, t60886, t60887, t60890, t60893, t60899) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2923::<F>(t4696, t13732, t4483, t4471, t950, t14369, t49513, t4475, t49532, t4496, t48883, t959);
        let (t60903, t60904) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2924::<F>(t10523, t2933, t5790, t959, t14662, t193, t3216, t336, t4700, t4701, t59891, t59958, t59961, t59966, t59968, t59970, t59972, t60880, t60886, t60890, t60893, t60899);
        let (t60906, t60908, t60909) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2925::<F>(t17934, t2944, t10623, t5804, t59981, t60006, t60008, t60010, t60016, t60021, t60023, t60025, t60027, t60029, t60033, t60035);
    (t60873, t60878, t60886, t60887, t60890, t60893, t60899, t60903, t60904, t60906, t60908, t60909)
}
