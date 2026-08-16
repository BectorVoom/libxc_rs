//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta544 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1993;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta544(t1882: f64, t9994: f64, t13872: f64, t221: f64, t4056: f64, t13867: f64, t13824: f64, t1398: f64, t5658: f64, t48073: f64, t543: f64, t3923: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t48141, t48525, t48662, t49306, t49376, t49380, t49393) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1993(t1882, t9994, t13872, t221, t4056, t13867, t13824, t1398, t5658, t48073, t543, t3923);
    (t48141, t48525, t48662, t49306, t49376, t49380, t49393)
}
