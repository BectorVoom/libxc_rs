//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta636 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2204;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2205;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta636<F: Float>(t1916: F, t26120: F, t26127: F, t26130: F, t1459: F, t28265: F, t26124: F, t28264: F, t4292: F, t572: F, t13514: F, t7330: F, t1518: F, t1936: F, t2371: F, t670: F, t7002: F, t4158: F, t7953: F, t101469: F, t117: F, t2327: F, t7741: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t101568, t101570, t101572, t101576, t101578, t101583, t101586) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2204::<F>(t1916, t26120, t26127, t26130, t1459, t28265, t26124, t28264, t4292, t572, t13514, t7330);
        let (t101590, t101594, t101598, t101601, t101606) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2205::<F>(t1518, t1936, t2371, t572, t670, t7002, t4158, t7953, t101469, t117, t2327, t7741);
    (t101568, t101570, t101572, t101576, t101578, t101583, t101586, t101590, t101594, t101598, t101601, t101606)
}
