//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta314 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1116;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1117;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1118;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1119;
use chunk4::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1120;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta314<F: Float>(t12088: F, t2535: F, t2504: F, t2368: F, t746: F, t1294: F, t268: F, t676: F, t9478: F, t9482: F, t9474: F, t9821: F, t2409: F, t2413: F, t125: F, t39253: F, t2414: F, t9479: F, t25: F, t11985: F, t526: F, t3665: F, t2249: F, t12061: F, t12064: F, t3664: F, t39109: F, t514: F, t9257: F, t11998: F, t528: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t39388, t39389, t39391, t39393, t39397) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1116::<F>(t12088, t2535, t2504, t2368, t746, t1294, t268, t676, t9478, t9482);
        let t39400 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1117::<F>(t268, t9474, t9821);
        let t39408 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1118::<F>(t2409, t2413, t125, t39253);
        let t39411 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1119::<F>(t2414, t39253, t9479);
        let (t39420, t39426, t39434, t39436) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1120::<F>(t25, t11985, t526, t3665, t2249, t12061, t12064, t3664, t39109, t514, t9257, t11998, t528, zeta_threshold);
    (t39388, t39389, t39391, t39393, t39397, t39400, t39408, t39411, t39420, t39426, t39434, t39436)
}
