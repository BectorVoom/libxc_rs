//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta374 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1391;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1392;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1393;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta374<F: Float>(t3247: F, t39103: F, t1113: F, t136: F, t11545: F, t241: F, t3241: F, t39097: F, t11229: F, t699: F, t11232: F, t11219: F, t43732: F, t242: F, t281: F, t415: F, t2394: F, t3253: F, t3249: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t43757, t43759, t43763, t43764, t43766, t43768, t43770, t43773) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1391::<F>(t3247, t39103, t1113, t136, t11545, t241, t3241, t39097, t11229, t699, t11232, t11219, t43732);
        let (t43776, t43777, t43780) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1392::<F>(t242, t281, t415, t2394, t3253);
        let t43782 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1393::<F>(t2394, t3249);
    (t43757, t43759, t43763, t43764, t43766, t43768, t43770, t43773, t43776, t43777, t43780, t43782)
}
