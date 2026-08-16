//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta427 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1490;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta427<F: Float>(t31027: F, t31424: F, t31440: F, t31032: F, t31444: F, t108: F, t1513: F, t116912: F, t31417: F, t31421: F, t2204: F, t5808: F) -> (F, F, F, F, F, F, F) {
        let (t117943, t117976, t117978, t117997, t118009, t118011, t118089) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1490::<F>(t31027, t31424, t31440, t31032, t31444, t108, t1513, t116912, t31417, t31421, t2204, t5808);
    (t117943, t117976, t117978, t117997, t118009, t118011, t118089)
}
