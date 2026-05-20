//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta638 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2412;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2413;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta638<F: Float>(t123: F, t2465: F, t886: F, t9291: F, t10982: F, t860: F, t9646: F, t2434: F, t2828: F, t10115: F, t251: F, t887: F, t2439: F, t2440: F, t2829: F, t10977: F, t686: F, t72: F, t11061: F, t11064: F, t2410: F, t2832: F, t775: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t41102, t41105, t41115, t41117, t41118) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2412::<F>(t123, t2465, t886, t9291, t10982, t860, t9646, t2434, t2828, t10115, t251, t887);
        let (t41125, t41129, t41137, t41154, t41161) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2413::<F>(t2439, t2440, t2829, t10977, t2465, t686, t72, t11061, t11064, t2410, t2832, t775);
    (t41102, t41105, t41115, t41117, t41118, t41125, t41129, t41137, t41154, t41161)
}
