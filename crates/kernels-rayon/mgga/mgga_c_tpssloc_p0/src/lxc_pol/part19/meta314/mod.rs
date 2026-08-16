//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta314 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1116;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1117;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1118;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1119;
use chunk4::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1120;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta314(t12088: f64, t2535: f64, t2504: f64, t2368: f64, t746: f64, t1294: f64, t268: f64, t676: f64, t9478: f64, t9482: f64, t9474: f64, t9821: f64, t2409: f64, t2413: f64, t125: f64, t39253: f64, t2414: f64, t9479: f64, t25: f64, t11985: f64, t526: f64, t3665: f64, t2249: f64, t12061: f64, t12064: f64, t3664: f64, t39109: f64, t514: f64, t9257: f64, t11998: f64, t528: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t39388, t39389, t39391, t39393, t39397) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1116(t12088, t2535, t2504, t2368, t746, t1294, t268, t676, t9478, t9482);
        let t39400 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1117(t268, t9474, t9821);
        let t39408 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1118(t2409, t2413, t125, t39253);
        let t39411 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1119(t2414, t39253, t9479);
        let (t39420, t39426, t39434, t39436) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1120(t25, t11985, t526, t3665, t2249, t12061, t12064, t3664, t39109, t514, t9257, t11998, t528, zeta_threshold);
    (t39388, t39389, t39391, t39393, t39397, t39400, t39408, t39411, t39420, t39426, t39434, t39436)
}
