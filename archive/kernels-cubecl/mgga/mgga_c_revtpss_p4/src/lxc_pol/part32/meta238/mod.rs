//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta238 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1012;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta238<F: Float>(t3357: F, t5044: F, t6423: F, t6427: F, t6431: F, t422: F, t1733: F, t5063: F, t1732: F, t1150: F, t3384: F, t1723: F) -> (F, F, F, F, F, F, F) {
        let (t6433, t6435, t6437, t6438, t6439, t6441, t6442) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1012::<F>(t3357, t5044, t6423, t6427, t6431, t422, t1733, t5063, t1732, t1150, t3384, t1723);
    (t6433, t6435, t6437, t6438, t6439, t6441, t6442)
}
