//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta526 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1854;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta526<F: Float>(t7063: F, t94878: F, t25877: F, t94801: F, t1419: F, t786: F, t2453: F, t25949: F, t25898: F, t112: F, t843: F, t239: F, t655: F) -> (F, F, F, F, F, F, F, F) {
        let (t94879, t94886, t94890, t94894, t94913, t94921, t94973, t94975) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1854::<F>(t7063, t94878, t25877, t94801, t1419, t786, t2453, t25949, t25898, t112, t843, t239, t655);
    (t94879, t94886, t94890, t94894, t94913, t94921, t94973, t94975)
}
