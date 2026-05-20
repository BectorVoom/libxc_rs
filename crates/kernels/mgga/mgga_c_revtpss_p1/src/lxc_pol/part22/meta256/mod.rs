//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta256 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1582;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1583;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1584;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1585;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta256<F: Float>(t1120: F, t6429: F, t128: F, t3357: F, t5044: F, t6423: F, t6427: F, t422: F, t1733: F, t5063: F, t1732: F, t1150: F, t3384: F, t1723: F, t3390: F, t3394: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t6430, t6431) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1582::<F>(t1120, t6429, t128);
        let (t6433, t6435, t6437, t6438, t6439) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1583::<F>(t3357, t5044, t6423, t6427, t6431, t422, t1733, t5063, t1732, t1150);
        let (t6441, t6442) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1584::<F>(t3384, t6439, t1723);
        let (t6443, t6449) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1585::<F>(t3390, t6442, t3394, t5044, t6423, t6427, t6431);
    (t6430, t6431, t6433, t6435, t6437, t6438, t6439, t6441, t6442, t6443, t6449)
}
