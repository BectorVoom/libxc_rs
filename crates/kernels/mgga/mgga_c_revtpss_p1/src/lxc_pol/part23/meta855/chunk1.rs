//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2744/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2744<F: Float>(t12916: F, t20805: F, t5331: F, t12784: F, t21090: F, t1222: F, t20293: F, t57484: F, t17735: F, t70646: F, t17423: F, t21014: F) -> (F, F, F, F, F) {
    let t71974 = t5331 * t12916 * t20805;
    let t71976 = t12784 * t21090;
    let t72000 = t1222 * t57484 * t20293;
    let t72002 = t17735 * t70646;
    let t72005 = t21014 * t17423;
    (t71974, t71976, t72000, t72002, t72005)
}
