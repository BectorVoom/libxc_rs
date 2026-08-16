//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta466 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1623;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1624;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta466<F: Float>(t25374: F, t25927: F, t1081: F, t1530: F, t28: F, t4303: F, t1649: F, t776: F, t868: F, t1307: F, t1845: F, t645: F, t72: F, t7431: F, t1437: F, t1864: F, t1410: F, t2240: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t25928, t25930, t25934, t25938, t25945, t25988, t26009) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1623::<F>(t25374, t25927, t1081, t1530, t28, t4303, t1649, t776, t868, t1307, t1845, t645, t72, t7431);
        let (t26012, t26016) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1624::<F>(t1437, t1864, t1410, t2240);
    (t25928, t25930, t25934, t25938, t25945, t25988, t26009, t26012, t26016)
}
