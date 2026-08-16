//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta303 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1196;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1197;
use chunk2::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1198;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta303(t10039: f64, t869: f64, t689: f64, t2777: f64, t4092: f64, t2439: f64, t3923: f64, t555: f64, t4003: f64, t5744: f64, t2782: f64, t4086: f64, t543: f64, t123: f64, t212: f64, t2434: f64, t4089: f64, t138: f64, t2438: f64, t785: f64, t1398: f64, t1419: f64, t4056: f64, t1432: f64, t2470: f64, t4107: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10041, t10044, t10062, t10065) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1196(t10039, t869, t689, t2777, t4092, t2439, t3923, t555, t4003, t5744, t2782, t4086, t543);
        let (t10066, t10069, t10070, t10073) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1197(t10065, t2782, t123, t212, t2434, t4089, t138, t2438, t785);
        let (t10074, t10080, t10085, t10098) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1198(t10073, t4089, t1398, t1419, t4086, t543, t2782, t4056, t555, t1432, t2470, t4107);
    (t10041, t10044, t10062, t10066, t10069, t10070, t10073, t10074, t10080, t10085, t10098)
}
