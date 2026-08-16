//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta240 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk970;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta240<F: Float>(t11148: F, t11219: F, t136: F, t11154: F, t3297: F, t11161: F, t11170: F, t11195: F, t11197: F, t11200: F, t11204: F, t11206: F, t11209: F, t11211: F, t11213: F, t11215: F, t11217: F) -> (F, F, F, F, F) {
        let (t11220, t11221, t11223, t11224, t11228) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk970::<F>(t11148, t11219, t136, t11154, t3297, t11161, t11170, t11195, t11197, t11200, t11204, t11206, t11209, t11211, t11213, t11215, t11217);
    (t11220, t11221, t11223, t11224, t11228)
}
