//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta473 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1454;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1455;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1456;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta473<F: Float>(t2439: F, t2440: F, t6072: F, t15003: F, t51258: F, t6042: F, t786: F, t867: F, t14485: F, t14987: F, t2435: F, t6093: F, t6097: F, t6101: F) -> (F, F, F, F, F, F, F) {
        let (t63050, t63058, t63084, t63099, t63453) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1454::<F>(t2439, t2440, t6072, t15003, t51258, t6042, t786, t867, t14485, t14987, t2435, t6093);
        let t63459 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1455::<F>(t2435, t6097);
        let t63464 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1456::<F>(t2435, t6101);
    (t63050, t63058, t63084, t63099, t63453, t63459, t63464)
}
