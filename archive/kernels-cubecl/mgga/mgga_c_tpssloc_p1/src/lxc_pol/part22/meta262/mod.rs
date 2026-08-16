//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta262 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1404;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1405;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta262<F: Float>(t11818: F, t1216: F, t248: F, t1213: F, t11552: F, t221: F, t456: F, t1197: F, t698: F, t1174: F, t1176: F, t3242: F, t10471: F, t11715: F, t11712: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t11820, t11821, t11832, t11834, t11835, t11836, t11848) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1404::<F>(t11818, t1216, t248, t1213, t11552, t221, t456, t1197, t698, t1174, t1176, t3242);
        let (t11880, t11881) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1405::<F>(t10471, t11715, t11712);
    (t11820, t11821, t11832, t11834, t11835, t11836, t11848, t11880, t11881)
}
