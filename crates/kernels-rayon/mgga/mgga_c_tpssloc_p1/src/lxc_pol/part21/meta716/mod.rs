//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta716 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2556;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2557;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta716(t1041: f64, t4584: f64, t49850: f64, t10422: f64, t14032: f64, t3070: f64, t13969: f64, t14166: f64, t14159: f64, t2960: f64, t14146: f64, t14068: f64, t10263: f64, t4603: f64, t10891: f64, t13970: f64, t10231: f64, t13528: f64, t973: f64, t13532: f64, t13537: f64, t42972: f64, t135: f64, t14197: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t50047, t50056, t50062, t50077, t50084, t50094) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2556(t1041, t4584, t49850, t10422, t14032, t3070, t13969, t14166, t14159, t2960, t14146, t14068);
        let (t50098, t50100, t50110, t50113, t50116, t50132) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2557(t10263, t4603, t10891, t13970, t10231, t13528, t973, t13532, t13537, t42972, t135, t14197);
    (t50047, t50056, t50062, t50077, t50084, t50094, t50098, t50100, t50110, t50113, t50116, t50132)
}
