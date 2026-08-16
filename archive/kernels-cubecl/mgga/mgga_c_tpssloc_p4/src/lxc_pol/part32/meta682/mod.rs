//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta682 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2122;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta682<F: Float>(t27930: F, t576: F, t112: F, t27907: F, t111: F, t8110: F, t28821: F, t6997: F, t1441: F, t4072: F, t1874: F, t28002: F, t6525: F) -> (F, F, F, F, F, F, F) {
        let (t96308, t96311, t96334, t96355, t96356, t96358, t96360) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2122::<F>(t27930, t576, t112, t27907, t111, t8110, t28821, t6997, t1441, t4072, t1874, t28002, t6525);
    (t96308, t96311, t96334, t96355, t96356, t96358, t96360)
}
