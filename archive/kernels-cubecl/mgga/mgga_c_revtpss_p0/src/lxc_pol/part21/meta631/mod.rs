//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta631 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2398;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2399;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta631<F: Float>(t10766: F, t10811: F, t10788: F, t14923: F, t10799: F, t10759: F, t2735: F, t40628: F, t854: F, t10890: F, t2707: F, t10896: F, t2703: F, t10293: F, t240: F, t243: F, t813: F, t816: F, t10675: F, t2689: F, t10777: F, t10779: F, t2706: F, t837: F, t798: F, t9726: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t40816, t40822, t40824, t40834, t40836, t40838, t40840) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2398::<F>(t10766, t10811, t10788, t14923, t10799, t10759, t2735, t40628, t854, t10890, t2707, t10896, t2703);
        let (t40846, t40850, t40851, t40855, t40861) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2399::<F>(t10293, t240, t243, t813, t816, t10675, t2689, t10777, t10779, t2706, t837, t798, t9726);
    (t40816, t40822, t40824, t40834, t40836, t40838, t40840, t40846, t40850, t40851, t40855, t40861)
}
