//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta316 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1577;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta316<F: Float>(t11516: F, t9288: F, t3440: F, t3441: F, t1177: F, t1178: F, t9258: F, t1176: F, t698: F) -> (F, F, F, F, F, F, F) {
        let (t11517, t11518, t11521, t11522, t11525, t11526, t11529) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1577::<F>(t11516, t9288, t3440, t3441, t1177, t1178, t9258, t1176, t698);
    (t11517, t11518, t11521, t11522, t11525, t11526, t11529)
}
