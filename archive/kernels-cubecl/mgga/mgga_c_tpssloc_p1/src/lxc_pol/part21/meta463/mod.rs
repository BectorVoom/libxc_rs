//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta463 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2032;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2033;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2034;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta463<F: Float>(t16081: F, t5198: F, t213: F, t5187: F, t1307: F, t221: F, t3719: F, t5196: F, t3732: F, t67: F, t792: F, t1799: F, t212: F, t686: F, t12214: F, t131: F, t205: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t16083, t16086, t16090, t16093, t16094) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2032::<F>(t16081, t5198, t213, t5187, t1307, t221, t3719, t5196, t3732, t67, t792);
        let t16095 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2033::<F>(t1799, t212);
        let (t16097, t16099, t16100, t16101) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2034::<F>(t1307, t16095, t686, t16094, t12214, t131, t205);
    (t16083, t16086, t16090, t16093, t16094, t16095, t16097, t16099, t16100, t16101)
}
