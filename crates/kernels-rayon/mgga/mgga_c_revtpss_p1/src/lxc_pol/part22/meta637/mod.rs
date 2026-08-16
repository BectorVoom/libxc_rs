//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta637 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2567;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2568;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2569;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta637(t12256: f64, t5819: f64, t606: f64, t12305: f64, t128: f64, t12268: f64, t3360: f64, t4186: f64, t5046: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t20292, t20293, t20294, t20295) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2567(t12256, t5819, t606, t12305, t128);
        let (t20297, t20298, t20299, t20300) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2568(t12268, t5819, t606, t3360, t128);
        let (t20302, t20303, t20304) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2569(t4186, t5046, t3360, t128);
    (t20292, t20293, t20294, t20295, t20297, t20298, t20299, t20300, t20302, t20303, t20304)
}
