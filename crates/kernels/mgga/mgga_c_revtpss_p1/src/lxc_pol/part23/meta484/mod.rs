//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta484 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1949;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1950;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1951;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta484<F: Float>(t12256: F, t5819: F, t606: F, t12305: F, t128: F, t12268: F, t3360: F, t4186: F, t5046: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t20292, t20293, t20294, t20295) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1949::<F>(t12256, t5819, t606, t12305, t128);
        let (t20297, t20298, t20299, t20300) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1950::<F>(t12268, t5819, t606, t3360, t128);
        let (t20302, t20303, t20304) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1951::<F>(t4186, t5046, t3360, t128);
    (t20292, t20293, t20294, t20295, t20297, t20298, t20299, t20300, t20302, t20303, t20304)
}
