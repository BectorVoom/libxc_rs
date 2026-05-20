//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta257 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk984;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk985;
use chunk2::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk986;
use chunk3::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk987;
use chunk4::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk988;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta257<F: Float>(t2501: F, t9285: F, t685: F, t793: F, t684: F, t125: F, t123: F, t128: F, t121: F, t22: F, t2508: F, t692: F, t124: F, t624: F, t138: F, t9283: F, t701: F, t682: F, t2580: F, t680: F, t130: F, t146: F, t2583: F, t9275: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t9286, t9288) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk984::<F>(t2501, t9285, t685, t793);
        let (t9289, t9291, t9292) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk985::<F>(t684, t9288, t125, t793, t123);
        let (t9296, t9298, t9300, t9303) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk986::<F>(t128, t121, t22, t2508, t9285, t692, t9288, t124, t624, t138);
        let t9308 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk987::<F>(t9283, t9286, t9289, t9292, t9296, t9298, t9300, t9303, t701, t682);
        let t9316 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk988::<F>(t2580, t680, t130, t146, t2583, t9275);
    (t9286, t9288, t9289, t9291, t9292, t9296, t9298, t9300, t9303, t9308, t9316)
}
