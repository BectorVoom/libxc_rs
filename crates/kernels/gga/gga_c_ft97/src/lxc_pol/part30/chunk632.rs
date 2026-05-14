//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 632/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk632<F: Float>(t7114: F, t875: F, t24898: F, t4176: F, t15369: F, t1476: F, t2842: F) -> (F, F, F, F) {
    let t29047 = t7114 * t875;
    let t29051 = t24898 * t4176;
    let t29052 = t15369 * t29051;
    let t29055 = t2842 * t1476;
    (t29047, t29051, t29052, t29055)
}
