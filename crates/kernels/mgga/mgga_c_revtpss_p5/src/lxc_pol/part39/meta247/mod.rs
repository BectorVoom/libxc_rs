//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta247 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk931;
use chunk1::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk932;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta247<F: Float>(t30: F, t189: F, t5566: F, t512: F, t1856: F, t749: F, t177: F, t762: F, t1468: F, t3874: F, t1344: F, t2: F, t580: F, t605: F, zeta_threshold: F, t33: F, t1711: F, t3881: F, t1348: F, t1113: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t5567, t5568, t5569, t5570, t5571, t5573, t5574, t5577, t5581) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk931::<F>(t30, t189, t5566, t512, t1856, t749, t177, t762, t1468, t3874, t1344, t2, t580, t605, zeta_threshold);
        let (t5582, t5585, t5591) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk932::<F>(t33, t1711, t3881, t1348, t2, t1113, t580, t5581, zeta_threshold);
    (t5567, t5568, t5569, t5570, t5571, t5573, t5574, t5577, t5582, t5585, t5591)
}
