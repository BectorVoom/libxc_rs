//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta533 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1948;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1949;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1950;
use chunk3::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1951;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta533<F: Float>(t5273: F, t7617: F, t5291: F, t7616: F, t1241: F, t5265: F, t7618: F, t1219: F, t8172: F, t5357: F, t7607: F, t5378: F, t7624: F, t1785: F, t7623: F, t3670: F, t2133: F, t816: F, t1224: F, t65: F, t5052: F, t1266: F, t1808: F, t26821: F, t26822: F, t26832: F, t26836: F, t26852: F, t26867: F, t5386: F, t5407: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t29010, t29019, t29020, t29023, t29027, t29031, t29034) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1948::<F>(t5273, t7617, t5291, t7616, t1241, t5265, t7618, t1219, t8172, t5357, t7607, t5378, t7624);
        let (t29037, t29040) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1949::<F>(t1785, t7623, t3670);
        let t29047 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1950::<F>(t2133, t816);
        let (t29048, t29052) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1951::<F>(t1224, t65, t5052, t1266, t1808, t26821, t26822, t26832, t26836, t26852, t26867, t29031, t29034, t29037, t29040, t29047, t5386, t5407);
    (t29010, t29019, t29020, t29023, t29027, t29037, t29040, t29047, t29048, t29052)
}
