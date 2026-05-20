//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta145 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk767;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta145<F: Float>(t2630: F, t3869: F, t1337: F, t2619: F, t514: F, t517: F, t1359: F, t2435: F, t555: F, t785: F, t1358: F, t2439: F) -> (F, F, F, F, F, F, F, F) {
        let (t3871, t3873, t3874, t3881, t3894, t3895, t3896, t3898) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk767::<F>(t2630, t3869, t1337, t2619, t514, t517, t1359, t2435, t555, t785, t1358, t2439);
    (t3871, t3873, t3874, t3881, t3894, t3895, t3896, t3898)
}
