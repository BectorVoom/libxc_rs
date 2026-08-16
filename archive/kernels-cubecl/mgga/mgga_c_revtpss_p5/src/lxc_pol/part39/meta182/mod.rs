//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta182 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk784;
use chunk1::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk785;
use chunk2::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk786;
use chunk3::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk787;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta182<F: Float>(t239: F, t4000: F, t820: F, t543: F, t3923: F, t1390: F, t828: F, t531: F, t549: F, t240: F, t72: F, t3829: F, t1386: F, t2482: F, t27: F, t136: F, t1389: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t4002, t4003) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk784::<F>(t239, t4000, t820, t543);
        let t4004 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk785::<F>(t3923, t4003);
        let (t4006, t4010, t4011, t4012, t4014, t4018) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk786::<F>(t1390, t4004, t828, t531, t549, t240, t72, t3829, t1386, t2482, t27);
        let t4019 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk787::<F>(t136, t1389);
    (t4002, t4003, t4004, t4006, t4010, t4011, t4012, t4014, t4018, t4019)
}
