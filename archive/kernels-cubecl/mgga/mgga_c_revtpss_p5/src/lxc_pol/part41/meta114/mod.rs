//! MGGA_C_REVTPSS lxc pol kernel — _part41_v4rho3tau_4 meta114 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk583;
use chunk1::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk584;
use chunk2::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk585;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta114<F: Float>(t1065: F, t159: F, t631: F, t2297: F) -> (F, F, F, F) {
        let t2850 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk583::<F>(t1065, t159);
        let (t2851, t2852) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk584::<F>(t631);
        let t2857 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk585::<F>(t2297);
    (t2850, t2851, t2852, t2857)
}
