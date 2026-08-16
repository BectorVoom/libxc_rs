//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta549 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1988;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta549(t1882: f64, t4056: f64, t13867: f64, t221: f64, t13824: f64, t1398: f64, t5658: f64, t48073: f64, t543: f64, t3923: f64, t48105: f64, t14304: f64, t4147: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t48525, t48662, t49306, t49376, t49380, t49393, t49564) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1988(t1882, t4056, t13867, t221, t13824, t1398, t5658, t48073, t543, t3923, t48105, t14304, t4147);
    (t48525, t48662, t49306, t49376, t49380, t49393, t49564)
}
