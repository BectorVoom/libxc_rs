//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta535 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1887;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1888;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta535(t30: f64, t265: f64, t393: f64, t27254: f64, t27256: f64, t28034: f64, t27924: f64, t27926: f64, t27929: f64, t27937: f64, t27955: f64, t27754: f64, t1469: f64, t2129: f64, t27408: f64, t4186: f64, t45: f64, t606: f64, t7594: f64, t8161: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64, t5273: f64, t7617: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t28336, t28337, t28679, t28872, t28873, t28874, t28877, t28885, t28998, t29005) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1887(t30, t265, t393, t27254, t27256, t28034, t27924, t27926, t27929, t27937, t27955, t27754, t1469, t2129, t27408, t4186, t45, t606, t7594, t8161, dens_threshold, rho0, zeta_threshold);
        let t29010 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1888(t5273, t7617);
    (t28336, t28337, t28679, t28872, t28873, t28874, t28877, t28885, t28998, t29005, t29010)
}
