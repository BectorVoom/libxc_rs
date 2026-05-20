//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta285 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1034;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta285<F: Float>(t10199: F, t112: F, t2289: F, t666: F, t2341: F, t625: F, t2367: F, t654: F, t98: F, t99: F, t106: F, t107: F) -> (F, F, F, F, F, F, F, F) {
        let (t10201, t10202, t10204, t10206, t10207, t10208, t10227, t10240) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1034::<F>(t10199, t112, t2289, t666, t2341, t625, t2367, t654, t98, t99, t106, t107);
    (t10201, t10202, t10204, t10206, t10207, t10208, t10227, t10240)
}
