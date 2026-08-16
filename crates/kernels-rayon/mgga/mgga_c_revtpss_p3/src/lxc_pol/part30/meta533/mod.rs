//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta533 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1948;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1949;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1950;
use chunk3::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1951;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta533(t5273: f64, t7617: f64, t5291: f64, t7616: f64, t1241: f64, t5265: f64, t7618: f64, t1219: f64, t8172: f64, t5357: f64, t7607: f64, t5378: f64, t7624: f64, t1785: f64, t7623: f64, t3670: f64, t2133: f64, t816: f64, t1224: f64, t65: f64, t5052: f64, t1266: f64, t1808: f64, t26821: f64, t26822: f64, t26832: f64, t26836: f64, t26852: f64, t26867: f64, t5386: f64, t5407: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t29010, t29019, t29020, t29023, t29027, t29031, t29034) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1948(t5273, t7617, t5291, t7616, t1241, t5265, t7618, t1219, t8172, t5357, t7607, t5378, t7624);
        let (t29037, t29040) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1949(t1785, t7623, t3670);
        let t29047 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1950(t2133, t816);
        let (t29048, t29052) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1951(t1224, t65, t5052, t1266, t1808, t26821, t26822, t26832, t26836, t26852, t26867, t29031, t29034, t29037, t29040, t29047, t5386, t5407);
    (t29010, t29019, t29020, t29023, t29027, t29037, t29040, t29047, t29048, t29052)
}
