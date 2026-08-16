//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta184 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk759;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta184<F: Float>(t342: F, t4980: F, t3154: F, t3302: F, t1043: F, t4893: F, t1071: F, t1089: F, t1668: F, t378: F, t4866: F, t3316: F) -> (F, F, F, F, F, F, F) {
        let (t4981, t4982, t4983, t4984, t4988, t4992, t4995) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk759::<F>(t342, t4980, t3154, t3302, t1043, t4893, t1071, t1089, t1668, t378, t4866, t3316);
    (t4981, t4982, t4983, t4984, t4988, t4992, t4995)
}
