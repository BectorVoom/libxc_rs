//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta553 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1892;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1893;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta553<F: Float>(t25950: F, t26271: F, t10073: F, t25920: F, t26260: F, t25898: F, t7527: F, t94849: F, t94383: F, t96221: F, t213: F, t26333: F, t2453: F, t26264: F, t9676: F, t26072: F, t26231: F, t94921: F, t1444: F, t2102: F, t25929: F, t7496: F, t9692: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t96500, t96503, t96506, t96510, t96512) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1892::<F>(t25950, t26271, t10073, t25920, t26260, t25898, t7527, t94849, t94383, t96221, t213, t26333);
        let (t96515, t96516, t96527, t96542, t96546, t96549) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1893::<F>(t2453, t26264, t9676, t26072, t26271, t26231, t94921, t10073, t1444, t2102, t25929, t7496, t9692);
    (t96500, t96503, t96506, t96510, t96512, t96515, t96516, t96527, t96542, t96546, t96549)
}
