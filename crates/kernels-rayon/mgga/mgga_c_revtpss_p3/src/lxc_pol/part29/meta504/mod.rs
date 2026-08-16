//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta504 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1821;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1822;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta504(t1882: f64, t3923: f64, t4003: f64, t9994: f64, t13872: f64, t221: f64, t4056: f64, t13867: f64, t13824: f64, t1398: f64, t5658: f64, t543: f64, t14304: f64, t4147: f64, t1868: f64, t4135: f64, t116: f64, t13424: f64, t10871: f64, t1558: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t48074, t48105, t48141, t48475, t48525, t48662, t49146, t49306) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1821(t1882, t3923, t4003, t9994, t13872, t221, t4056, t13867, t13824, t1398, t5658, t543);
        let (t49376, t49380, t49393, t49564, t49582, t49686, t50474) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1822(t49146, t543, t48475, t3923, t48105, t14304, t4147, t1868, t4135, t116, t13424, t10871, t1558);
    (t48074, t48141, t48525, t48662, t49306, t49376, t49380, t49393, t49564, t49582, t49686, t50474)
}
