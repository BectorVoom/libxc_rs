//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta672 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2201;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2202;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta672<F: Float>(t109077: F, t2035: F, t22496: F, t25082: F, t33651: F, t29576: F, t7235: F, t2014: F, t22475: F, t7312: F, t2034: F, t73407: F, t30122: F, t32113: F, t1448: F, t6781: F, t28196: F, t98495: F, t1353: F, t28197: F, t28167: F, t8717: F, t25190: F, t29494: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t109078, t109081, t109087, t109090, t109092) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2201::<F>(t109077, t2035, t22496, t25082, t33651, t29576, t7235, t2014, t22475, t7312, t2034, t73407);
        let (t109095, t109099, t109103, t109107, t109110) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2202::<F>(t25082, t30122, t32113, t1448, t6781, t28196, t98495, t1353, t28197, t28167, t8717, t2014, t25190, t29494);
    (t109078, t109081, t109087, t109090, t109092, t109095, t109099, t109103, t109107, t109110)
}
