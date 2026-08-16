//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta485 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1952;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1953;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1954;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1955;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta485<F: Float>(t606: F, t6421: F, t1120: F, t128: F, t4186: F, t5051: F, t20266: F, t3360: F, t3367: F, t5825: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t20306, t20307, t20308) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1952::<F>(t606, t6421, t1120, t128);
        let (t20310, t20311, t20312) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1953::<F>(t4186, t5051, t1120, t128);
        let (t20314, t20315) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1954::<F>(t20266, t3360, t128);
        let (t20317, t20318, t20319, t20320) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1955::<F>(t3367, t5825, t606, t1120, t128);
    (t20306, t20307, t20308, t20310, t20311, t20312, t20314, t20315, t20317, t20318, t20319, t20320)
}
