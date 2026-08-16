//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta716 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2556;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2557;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta716<F: Float>(t1041: F, t4584: F, t49850: F, t10422: F, t14032: F, t3070: F, t13969: F, t14166: F, t14159: F, t2960: F, t14146: F, t14068: F, t10263: F, t4603: F, t10891: F, t13970: F, t10231: F, t13528: F, t973: F, t13532: F, t13537: F, t42972: F, t135: F, t14197: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t50047, t50056, t50062, t50077, t50084, t50094) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2556::<F>(t1041, t4584, t49850, t10422, t14032, t3070, t13969, t14166, t14159, t2960, t14146, t14068);
        let (t50098, t50100, t50110, t50113, t50116, t50132) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2557::<F>(t10263, t4603, t10891, t13970, t10231, t13528, t973, t13532, t13537, t42972, t135, t14197);
    (t50047, t50056, t50062, t50077, t50084, t50094, t50098, t50100, t50110, t50113, t50116, t50132)
}
