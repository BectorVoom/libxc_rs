//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta616 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1956;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta616<F: Float>(t22081: F, t26028: F, t22085: F, t22048: F, t27940: F, t22089: F, t22146: F, t26004: F, t6884: F, t6850: F, t94513: F, t22041: F, t7252: F) -> (F, F, F, F, F, F, F, F) {
        let (t108526, t108528, t108531, t108533, t108535, t108537, t108539, t108541) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1956::<F>(t22081, t26028, t22085, t22048, t27940, t22089, t22146, t26004, t6884, t6850, t94513, t22041, t7252);
    (t108526, t108528, t108531, t108533, t108535, t108537, t108539, t108541)
}
