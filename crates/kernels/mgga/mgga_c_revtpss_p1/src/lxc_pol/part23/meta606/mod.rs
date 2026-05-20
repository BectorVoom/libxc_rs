//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta606 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2263;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2264;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2265;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2266;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2267;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2268;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta606<F: Float>(t24232: F, t3360: F, t128: F, t5046: F, t5825: F, t22688: F, t3362: F, t1120: F, t5051: F, t1121: F, t22671: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t24233, t24234) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2263::<F>(t24232, t3360, t128);
        let (t24236, t24237, t24238) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2264::<F>(t5046, t5825, t3360, t128);
        let t24240 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2265::<F>(t22688, t3362);
        let (t24241, t24242) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2266::<F>(t1120, t24240, t128);
        let (t24244, t24245, t24246) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2267::<F>(t5051, t5825, t1120, t128);
        let t24248 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2268::<F>(t1121, t22671);
    (t24233, t24234, t24236, t24237, t24238, t24240, t24241, t24242, t24244, t24245, t24246, t24248)
}
