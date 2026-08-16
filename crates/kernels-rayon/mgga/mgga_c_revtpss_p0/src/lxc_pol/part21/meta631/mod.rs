//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta631 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2398;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2399;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta631(t10766: f64, t10811: f64, t10788: f64, t14923: f64, t10799: f64, t10759: f64, t2735: f64, t40628: f64, t854: f64, t10890: f64, t2707: f64, t10896: f64, t2703: f64, t10293: f64, t240: f64, t243: f64, t813: f64, t816: f64, t10675: f64, t2689: f64, t10777: f64, t10779: f64, t2706: f64, t837: f64, t798: f64, t9726: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t40816, t40822, t40824, t40834, t40836, t40838, t40840) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2398(t10766, t10811, t10788, t14923, t10799, t10759, t2735, t40628, t854, t10890, t2707, t10896, t2703);
        let (t40846, t40850, t40851, t40855, t40861) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2399(t10293, t240, t243, t813, t816, t10675, t2689, t10777, t10779, t2706, t837, t798, t9726);
    (t40816, t40822, t40824, t40834, t40836, t40838, t40840, t40846, t40850, t40851, t40855, t40861)
}
