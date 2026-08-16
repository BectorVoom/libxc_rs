//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta167 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk804;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta167<F: Float>(t1539: F, t248: F, t3051: F, t1041: F, t1616: F, t884: F, t3071: F, t1023: F, t247: F, t375: F) -> (F, F, F, F, F, F, F) {
        let (t4571, t4572, t4574, t4575, t4578, t4579, t4582) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk804::<F>(t1539, t248, t3051, t1041, t1616, t884, t3071, t1023, t247, t375);
    (t4571, t4572, t4574, t4575, t4578, t4579, t4582)
}
