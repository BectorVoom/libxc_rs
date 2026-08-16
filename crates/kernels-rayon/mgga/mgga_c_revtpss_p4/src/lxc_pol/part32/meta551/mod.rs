//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta551 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1867;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1868;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta551(t25898: f64, t7527: f64, t94849: f64, t94383: f64, t96221: f64, t2453: f64, t26264: f64, t9676: f64, t10073: f64, t1444: f64, t2102: f64, t25929: f64, t7496: f64, t9692: f64, t1445: f64, t2439: f64, t26358: f64, t26252: f64, t3920: f64, t26249: f64, t9664: f64, t25895: f64, t96264: f64, t1426: f64, t7507: f64, t786: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t96506, t96510, t96515, t96516, t96546) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1867(t25898, t7527, t94849, t94383, t96221, t2453, t26264, t9676, t10073, t1444, t2102, t25929);
        let (t96549, t96559, t96561, t96564, t96565, t96576) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1868(t7496, t9692, t1445, t2439, t26358, t26252, t3920, t26249, t9664, t25895, t96264, t1426, t7507, t786);
    (t96506, t96510, t96515, t96516, t96546, t96549, t96559, t96561, t96564, t96565, t96576)
}
