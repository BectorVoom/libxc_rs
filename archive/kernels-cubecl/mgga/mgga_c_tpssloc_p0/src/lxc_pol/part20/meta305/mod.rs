//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta305 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1549;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1550;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1551;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1552;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta305<F: Float>(t3298: F, t699: F, t3301: F, t3304: F, t241: F, t3439: F, t11148: F, t136: F, t11154: F, t3297: F, t11161: F, t11170: F, t11195: F, t11197: F, t11200: F, t11204: F, t11206: F, t11209: F, t11211: F) -> (F, F, F, F, F, F, F, F, F) {
        let t11213 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1549::<F>(t3298, t699);
        let t11215 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1550::<F>(t3301, t699);
        let t11217 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1551::<F>(t3304, t699);
        let (t11219, t11220, t11221, t11223, t11224, t11228) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1552::<F>(t241, t3439, t11148, t136, t11154, t3297, t11161, t11170, t11195, t11197, t11200, t11204, t11206, t11209, t11211, t11213, t11215, t11217);
    (t11213, t11215, t11217, t11219, t11220, t11221, t11223, t11224, t11228)
}
