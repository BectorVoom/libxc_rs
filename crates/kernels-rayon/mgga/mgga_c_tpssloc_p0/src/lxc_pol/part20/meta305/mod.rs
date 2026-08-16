//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta305 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1549;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1550;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1551;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1552;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta305(t3298: f64, t699: f64, t3301: f64, t3304: f64, t241: f64, t3439: f64, t11148: f64, t136: f64, t11154: f64, t3297: f64, t11161: f64, t11170: f64, t11195: f64, t11197: f64, t11200: f64, t11204: f64, t11206: f64, t11209: f64, t11211: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t11213 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1549(t3298, t699);
        let t11215 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1550(t3301, t699);
        let t11217 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1551(t3304, t699);
        let (t11219, t11220, t11221, t11223, t11224, t11228) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1552(t241, t3439, t11148, t136, t11154, t3297, t11161, t11170, t11195, t11197, t11200, t11204, t11206, t11209, t11211, t11213, t11215, t11217);
    (t11213, t11215, t11217, t11219, t11220, t11221, t11223, t11224, t11228)
}
