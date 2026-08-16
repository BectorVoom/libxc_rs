//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta485 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1971;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta485<F: Float>(t1799: F, t212: F, t1307: F, t686: F, t16094: F, t12214: F, t131: F, t205: F, t221: F, t3734: F, t5196: F, t3726: F, t5206: F) -> (F, F, F, F, F, F, F) {
        let (t16095, t16097, t16099, t16100, t16101, t16103, t16106) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1971::<F>(t1799, t212, t1307, t686, t16094, t12214, t131, t205, t221, t3734, t5196, t3726, t5206);
    (t16095, t16097, t16099, t16100, t16101, t16103, t16106)
}
