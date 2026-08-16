//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta178 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk745;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk746;
use chunk2::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk747;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta178<F: Float>(t373: F, t4772: F, t371: F, t372: F, t225: F, t4746: F, t366: F, t4589: F, t4592: F, t4594: F, t4597: F, t4634: F, t4638: F, t4716: F, t4718: F, t4721: F, t4723: F, t4727: F, t4731: F, t4736: F) -> (F, F, F, F, F) {
        let (t4852, t4854, t4857) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk745::<F>(t373, t4772, t371, t372, t225, t4746);
        let t4858 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk746::<F>(t366, t4857);
        let t4866 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk747::<F>(t4589, t4592, t4594, t4597, t4634, t4638, t4716, t4718, t4721, t4723, t4727, t4731, t4736);
    (t4852, t4854, t4857, t4858, t4866)
}
