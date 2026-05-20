//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta161 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk808;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk809;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta161<F: Float>(t1150: F, t6438: F, t3384: F, t1723: F, t3390: F, t3394: F, t5044: F, t6423: F, t6427: F, t6431: F) -> (F, F, F, F, F) {
        let (t6439, t6441, t6442) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk808::<F>(t1150, t6438, t3384, t1723);
        let (t6443, t6449) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk809::<F>(t3390, t6442, t3394, t5044, t6423, t6427, t6431);
    (t6439, t6441, t6442, t6443, t6449)
}
