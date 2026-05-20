//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta551 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1867;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1868;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta551<F: Float>(t25898: F, t7527: F, t94849: F, t94383: F, t96221: F, t2453: F, t26264: F, t9676: F, t10073: F, t1444: F, t2102: F, t25929: F, t7496: F, t9692: F, t1445: F, t2439: F, t26358: F, t26252: F, t3920: F, t26249: F, t9664: F, t25895: F, t96264: F, t1426: F, t7507: F, t786: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t96506, t96510, t96515, t96516, t96546) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1867::<F>(t25898, t7527, t94849, t94383, t96221, t2453, t26264, t9676, t10073, t1444, t2102, t25929);
        let (t96549, t96559, t96561, t96564, t96565, t96576) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1868::<F>(t7496, t9692, t1445, t2439, t26358, t26252, t3920, t26249, t9664, t25895, t96264, t1426, t7507, t786);
    (t96506, t96510, t96515, t96516, t96546, t96549, t96559, t96561, t96564, t96565, t96576)
}
