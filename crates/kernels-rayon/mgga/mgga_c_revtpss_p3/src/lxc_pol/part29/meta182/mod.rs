//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta182 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk856;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk857;
use chunk2::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk858;
use chunk3::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk859;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta182(t1390: f64, t3924: f64, t828: f64, t1386: f64, t820: f64, t843: f64, t1401: f64, t241: f64, t1412: f64, t72: f64, t245: f64, t125: f64, t1398: f64, t1353: f64, t543: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3926, t3930) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk856(t1390, t3924, t828, t1386, t820, t843);
        let (t3931, t3934) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk857(t1401, t3930, t1386, t241, t820);
        let (t3935, t3936) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk858(t1412, t72, t245);
        let (t3937, t3938) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk859(t125, t1398, t1353, t543);
    (t3926, t3930, t3931, t3934, t3935, t3936, t3937, t3938)
}
