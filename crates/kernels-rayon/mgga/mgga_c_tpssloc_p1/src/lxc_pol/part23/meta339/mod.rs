//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta339 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1114;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1115;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1116;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1117;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta339(t2511: f64, t39377: f64, t39378: f64, t1294: f64, t2504: f64, t2368: f64, t746: f64, t268: f64, t676: f64, t9478: f64, t9482: f64, t9474: f64, t9821: f64, t2409: f64, t2413: f64, t125: f64, t39253: f64, t2414: f64, t9479: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t39381, t39382, t39384, t39389, t39391, t39393, t39397) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1114(t2511, t39377, t39378, t1294, t2504, t2368, t746, t268, t676, t9478, t9482);
        let t39400 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1115(t268, t9474, t9821);
        let t39408 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1116(t2409, t2413, t125, t39253);
        let t39411 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1117(t2414, t39253, t9479);
    (t39381, t39382, t39384, t39389, t39391, t39393, t39397, t39400, t39408, t39411)
}
