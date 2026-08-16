//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta574 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1991;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta574<F: Float>(t1484: F, t2752: F, t13487: F, t2749: F, t4303: F, t868: F, t4119: F, t4233: F, t829: F, t16935: F, t828: F, t2745: F) -> (F, F, F, F, F, F, F) {
        let (t57912, t57921, t58009, t58071, t58300, t58345, t59580) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1991::<F>(t1484, t2752, t13487, t2749, t4303, t868, t4119, t4233, t829, t16935, t828, t2745);
    (t57912, t57921, t58009, t58071, t58300, t58345, t59580)
}
