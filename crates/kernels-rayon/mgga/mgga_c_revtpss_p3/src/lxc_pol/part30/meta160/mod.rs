//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta160 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk818;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta160(t1169: f64, t3453: f64, t3356: f64, t3413: f64, t3358: f64, t3365: f64, t3370: f64, t3374: f64, t3392: f64, t3400: f64, t3408: f64, t3410: f64, t3415: f64, t3419: f64, t3422: f64, t3425: f64) -> (f64, f64, f64, f64) {
        let (t3454, t3459, t3466, t3471) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk818(t1169, t3453, t3356, t3413, t3358, t3365, t3370, t3374, t3392, t3400, t3408, t3410, t3415, t3419, t3422, t3425);
    (t3454, t3459, t3466, t3471)
}
