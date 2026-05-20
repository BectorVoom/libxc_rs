//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta615 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1955;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta615<F: Float>(t22271: F, t27940: F, t22163: F, t6871: F, t94429: F, t22159: F, t98115: F, t22120: F, t26028: F, t22076: F, t22102: F, t94423: F) -> (F, F, F, F, F, F, F) {
        let (t108512, t108514, t108516, t108518, t108520, t108522, t108524) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1955::<F>(t22271, t27940, t22163, t6871, t94429, t22159, t98115, t22120, t26028, t22076, t22102, t94423);
    (t108512, t108514, t108516, t108518, t108520, t108522, t108524)
}
