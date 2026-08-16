//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta223 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk864;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk865;
use chunk2::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk866;
use chunk3::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk867;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta223<F: Float>(t45: F, t57: F, t4399: F, t5819: F, t5825: F, t766: F, t80: F, t770: F, t83: F, zeta_threshold: F, t1544: F, t4546: F, t1558: F, t231: F) -> (F, F, F, F, F, F) {
        let (t5948, t5962) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk864::<F>(t45, t57, t4399, t5819, t5825, t766, t80, t770, t83, zeta_threshold);
        let t5966 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk865::<F>(t1544);
        let (t5970, t5977) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk866::<F>(t1544, t4546, t1558);
        let t5978 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk867::<F>(t231, t5977);
    (t5948, t5962, t5966, t5970, t5977, t5978)
}
