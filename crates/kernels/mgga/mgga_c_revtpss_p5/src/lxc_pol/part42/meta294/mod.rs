//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta294 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1059;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta294<F: Float>(t11337: F, t240: F, t3252: F, t276: F, t285: F, t273: F, t2439: F, t931: F, t2922: F, t913: F, t275: F, t290: F, t2925: F) -> (F, F, F, F, F, F, F) {
        let (t11338, t11341, t11354, t11358, t11366, t11385, t11387) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1059::<F>(t11337, t240, t3252, t276, t285, t273, t2439, t931, t2922, t913, t275, t290, t2925);
    (t11338, t11341, t11354, t11358, t11366, t11385, t11387)
}
