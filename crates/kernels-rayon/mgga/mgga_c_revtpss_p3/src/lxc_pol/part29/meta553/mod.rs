//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta553 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1892;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1893;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta553(t25950: f64, t26271: f64, t10073: f64, t25920: f64, t26260: f64, t25898: f64, t7527: f64, t94849: f64, t94383: f64, t96221: f64, t213: f64, t26333: f64, t2453: f64, t26264: f64, t9676: f64, t26072: f64, t26231: f64, t94921: f64, t1444: f64, t2102: f64, t25929: f64, t7496: f64, t9692: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t96500, t96503, t96506, t96510, t96512) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1892(t25950, t26271, t10073, t25920, t26260, t25898, t7527, t94849, t94383, t96221, t213, t26333);
        let (t96515, t96516, t96527, t96542, t96546, t96549) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1893(t2453, t26264, t9676, t26072, t26271, t26231, t94921, t10073, t1444, t2102, t25929, t7496, t9692);
    (t96500, t96503, t96506, t96510, t96512, t96515, t96516, t96527, t96542, t96546, t96549)
}
