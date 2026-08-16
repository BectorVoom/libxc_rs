//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta636 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2204;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2205;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta636(t1916: f64, t26120: f64, t26127: f64, t26130: f64, t1459: f64, t28265: f64, t26124: f64, t28264: f64, t4292: f64, t572: f64, t13514: f64, t7330: f64, t1518: f64, t1936: f64, t2371: f64, t670: f64, t7002: f64, t4158: f64, t7953: f64, t101469: f64, t117: f64, t2327: f64, t7741: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t101568, t101570, t101572, t101576, t101578, t101583, t101586) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2204(t1916, t26120, t26127, t26130, t1459, t28265, t26124, t28264, t4292, t572, t13514, t7330);
        let (t101590, t101594, t101598, t101601, t101606) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2205(t1518, t1936, t2371, t572, t670, t7002, t4158, t7953, t101469, t117, t2327, t7741);
    (t101568, t101570, t101572, t101576, t101578, t101583, t101586, t101590, t101594, t101598, t101601, t101606)
}
