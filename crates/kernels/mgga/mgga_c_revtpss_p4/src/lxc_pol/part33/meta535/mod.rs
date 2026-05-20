//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta535 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1887;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1888;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta535<F: Float>(t30: F, t265: F, t393: F, t27254: F, t27256: F, t28034: F, t27924: F, t27926: F, t27929: F, t27937: F, t27955: F, t27754: F, t1469: F, t2129: F, t27408: F, t4186: F, t45: F, t606: F, t7594: F, t8161: F, dens_threshold: F, rho0: F, zeta_threshold: F, t5273: F, t7617: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t28336, t28337, t28679, t28872, t28873, t28874, t28877, t28885, t28998, t29005) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1887::<F>(t30, t265, t393, t27254, t27256, t28034, t27924, t27926, t27929, t27937, t27955, t27754, t1469, t2129, t27408, t4186, t45, t606, t7594, t8161, dens_threshold, rho0, zeta_threshold);
        let t29010 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1888::<F>(t5273, t7617);
    (t28336, t28337, t28679, t28872, t28873, t28874, t28877, t28885, t28998, t29005, t29010)
}
