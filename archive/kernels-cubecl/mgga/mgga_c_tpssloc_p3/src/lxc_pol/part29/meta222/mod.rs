//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta222 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1058;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta222<F: Float>(t4964: F, t68: F, t484: F, t1177: F, t4729: F, t1229: F, t3247: F, t3961: F, t4582: F, t1734: F, t486: F) -> (F, F, F, F, F, F, F) {
        let (t4965, t4966, t4969, t4972, t4973, t4974, t4977) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1058::<F>(t4964, t68, t484, t1177, t4729, t1229, t3247, t3961, t4582, t1734, t486);
    (t4965, t4966, t4969, t4972, t4973, t4974, t4977)
}
