//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta132 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk632;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk633;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta132<F: Float>(t1175: F, t300: F, t3356: F, t1203: F, t1208: F, t487: F, t1204: F, t1207: F, t458: F, t456: F) -> (F, F, F, F, F, F, F) {
        let (t3531, t3546, t3555) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk632::<F>(t1175, t300, t3356, t1203, t1208);
        let (t3556, t3561, t3565, t3566) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk633::<F>(t3555, t487, t1204, t1207, t458, t456);
    (t3531, t3546, t3555, t3556, t3561, t3565, t3566)
}
