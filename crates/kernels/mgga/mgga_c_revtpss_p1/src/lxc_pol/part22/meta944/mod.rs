//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta944 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3180;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3181;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta944<F: Float>(t3718: F, t44546: F, t5347: F, t12916: F, t17785: F, t5331: F, t3650: F, t5390: F, t12915: F, t16775: F, t247: F, t5384: F, t12948: F, t17377: F, t17361: F, t3708: F, t17290: F, t3678: F, t3625: F, t44250: F, t5401: F, t127: F, t5277: F, t12866: F, t3630: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t58850, t58853, t58863, t58868) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3180::<F>(t3718, t44546, t5347, t12916, t17785, t5331, t3650, t5390, t12915, t16775, t247, t5384);
        let (t58878, t58882, t58884, t58889, t58895, t58897) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3181::<F>(t12948, t17377, t17361, t3708, t17290, t3678, t3625, t44250, t5401, t127, t5277, t12866, t3630);
    (t58850, t58853, t58863, t58868, t58878, t58882, t58884, t58889, t58895, t58897)
}
