//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2398/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2398<F: Float>(t10766: F, t10811: F, t10788: F, t14923: F, t10799: F, t10759: F, t2735: F, t40628: F, t854: F, t10890: F, t2707: F, t10896: F, t2703: F) -> (F, F, F, F, F, F, F) {
    let t40816 = t10811 * t10766;
    let t40822 = t14923 * t10788;
    let t40824 = t14923 * t10799;
    let t40834 = t2735 * t10759;
    let t40836 = t40834 * t854 * t40628;
    let t40838 = t10890 * t2707;
    let t40840 = t2703 * t10896;
    (t40816, t40822, t40824, t40834, t40836, t40838, t40840)
}
