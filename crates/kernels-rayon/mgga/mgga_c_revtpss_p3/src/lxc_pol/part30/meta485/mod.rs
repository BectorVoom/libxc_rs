//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta485 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1824;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1825;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1826;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta485(t1398: f64, t1444: f64, t543: f64, t25931: f64, t1426: f64, t545: f64, t2022: f64, t7282: f64, t10073: f64, t2453: f64, t7283: f64, t136: f64, t2029: f64, t2457: f64, t25920: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t25933, t25934, t25937, t25938, t25939, t25941, t25944, t25945) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1824(t1398, t1444, t543, t25931, t1426, t545, t2022, t7282, t10073, t2453, t7283, t136, t2029);
        let t25946 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1825(t2457, t25945);
        let (t25948, t25949) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1826(t25944, t25946, t1426, t25920);
    (t25933, t25934, t25937, t25938, t25939, t25941, t25944, t25945, t25946, t25948, t25949)
}
