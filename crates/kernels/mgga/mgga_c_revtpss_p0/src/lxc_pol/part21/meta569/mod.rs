//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta569 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2267;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2268;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2269;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta569<F: Float>(t372: F, t5302: F, t4181: F, t5405: F, t13396: F, t1042: F, t3588: F, t3603: F, t5332: F, t3720: F, t15904: F, t3623: F, t13148: F, t11249: F, t1794: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t17694, t17695, t17696, t17699, t17700, t17703, t17704, t17705, t17708) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2267::<F>(t372, t5302, t4181, t5405, t13396, t1042, t3588, t3603, t5332, t3720, t15904, t3623);
        let t17709 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2268::<F>(t13148, t17708);
        let t17710 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2269::<F>(t11249, t1794);
    (t17694, t17695, t17696, t17699, t17700, t17703, t17704, t17705, t17708, t17709, t17710)
}
