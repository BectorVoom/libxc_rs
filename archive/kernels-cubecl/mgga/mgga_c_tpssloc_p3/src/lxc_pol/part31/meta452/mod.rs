//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta452 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1603;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta452<F: Float>(t25084: F, t4184: F, t23146: F, t4191: F, t4240: F, t4250: F, t13228: F, t828: F, t2628: F, t6605: F, t13351: F, t232: F) -> (F, F, F, F, F, F, F, F) {
        let (t25085, t25087, t25089, t25091, t25093, t25094, t25095, t25097) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1603::<F>(t25084, t4184, t23146, t4191, t4240, t4250, t13228, t828, t2628, t6605, t13351, t232);
    (t25085, t25087, t25089, t25091, t25093, t25094, t25095, t25097)
}
