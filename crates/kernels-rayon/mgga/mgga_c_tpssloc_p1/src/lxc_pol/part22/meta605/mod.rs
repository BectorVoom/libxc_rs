//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta605 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2129;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2130;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta605(t1041: f64, t4589: f64, t49850: f64, t10969: f64, t41687: f64, t10868: f64, t248: f64, t4347: f64, t10224: f64, t4343: f64, t973: f64, t3130: f64, t4595: f64, t10402: f64, t14618: f64, t14608: f64, t10936: f64, t4669: f64, t3082: f64, t4617: f64, t4584: f64, t14159: f64, t2960: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t49853, t49854, t49872, t49907, t49922) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2129(t1041, t4589, t49850, t10969, t41687, t10868, t248, t4347, t10224, t4343, t973, t3130, t4595);
        let (t49923, t49929, t49934, t49984, t49994, t50048, t50077) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2130(t49922, t10402, t14618, t14608, t10936, t4669, t3082, t4617, t1041, t4584, t49850, t14159, t2960);
    (t49853, t49854, t49872, t49907, t49923, t49929, t49934, t49984, t49994, t50048, t50077)
}
