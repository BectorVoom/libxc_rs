//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta243 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1084;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1085;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1086;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta243<F: Float>(t1294: F, t1828: F, t3737: F, t1284: F, t1770: F, t1280: F, t5230: F, t1287: F, t5346: F, t1774: F, t3759: F, t5245: F, t354: F, t471: F, t1214: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t5428, t5429) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1084::<F>(t1294, t1828, t3737);
        let (t5436, t5443, t5446, t5449, t5452, t5457) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1085::<F>(t1284, t1770, t1280, t5230, t1287, t5346, t1774, t3759, t5245, t354, t471);
        let t5458 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1086::<F>(t1214, t5457);
    (t5428, t5429, t5436, t5443, t5446, t5449, t5452, t5457, t5458)
}
