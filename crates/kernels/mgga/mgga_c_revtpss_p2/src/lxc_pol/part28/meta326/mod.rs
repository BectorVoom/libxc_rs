//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta326 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1338;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1339;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta326<F: Float>(t2729: F, t794: F, t2732: F, t136: F, t860: F, t2457: F, t2710: F, t10652: F, t231: F, t2783: F, t2782: F, t10069: F, t2786: F, t10073: F, t836: F, t251: F, t2645: F, t10111: F, t22: F, t870: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t10905, t10906, t10916, t10921, t10923) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1338::<F>(t2729, t794, t2732, t136, t860, t2457, t2710, t10652, t231, t2783, t2782, t10069, t2786);
        let (t10925, t10930, t10935, t10939) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1339::<F>(t10073, t2786, t231, t2783, t836, t860, t2782, t251, t2645, t10111, t22, t870);
    (t10905, t10906, t10916, t10921, t10923, t10925, t10930, t10935, t10939)
}
