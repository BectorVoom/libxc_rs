//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta287 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1273;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta287<F: Float>(t555: F, t9646: F, t1358: F, t22: F, t1425: F, t225: F, t3907: F, t9285: F, t3906: F, t2453: F, t3914: F, t1444: F, t2438: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t9648, t9650, t9655, t9656, t9657, t9664, t9666, t9674, t9675) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1273::<F>(t555, t9646, t1358, t22, t1425, t225, t3907, t9285, t3906, t2453, t3914, t1444, t2438);
    (t9648, t9650, t9655, t9656, t9657, t9664, t9666, t9674, t9675)
}
