//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta855 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2743;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2744;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta855<F: Float>(t17729: F, t20922: F, t44425: F, t17396: F, t17617: F, t1222: F, t6658: F, t697: F, t6662: F, t12916: F, t20801: F, t5340: F, t20805: F, t5331: F, t12784: F, t21090: F, t20293: F, t57484: F, t17735: F, t70646: F, t17423: F, t21014: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t71908, t71920, t71928, t71931, t71971) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2743::<F>(t17729, t20922, t44425, t17396, t17617, t1222, t6658, t697, t6662, t12916, t20801, t5340);
        let (t71974, t71976, t72000, t72002, t72005) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2744::<F>(t12916, t20805, t5331, t12784, t21090, t1222, t20293, t57484, t17735, t70646, t17423, t21014);
    (t71908, t71920, t71928, t71931, t71971, t71974, t71976, t72000, t72002, t72005)
}
