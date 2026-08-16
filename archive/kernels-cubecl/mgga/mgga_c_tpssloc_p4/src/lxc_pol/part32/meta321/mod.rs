//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta321 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1350;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta321<F: Float>(t11588: F, t1184: F, t1239: F, t68: F, t1203: F, t3540: F, t2393: F, t374: F, t486: F, t485: F, t3576: F, t3604: F) -> (F, F, F, F, F, F, F, F) {
        let (t11589, t11604, t11605, t11606, t11644, t11647, t11649, t11665) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1350::<F>(t11588, t1184, t1239, t68, t1203, t3540, t2393, t374, t486, t485, t3576, t3604);
    (t11589, t11604, t11605, t11606, t11644, t11647, t11649, t11665)
}
