//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta310 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1299;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1300;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1301;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta310<F: Float>(t10039: F, t869: F, t689: F, t2777: F, t4092: F, t2439: F, t3923: F, t555: F, t4003: F, t5744: F, t2782: F, t4086: F, t543: F, t123: F, t212: F, t2434: F, t4089: F, t138: F, t2438: F, t785: F, t1398: F, t1419: F, t4056: F, t1432: F, t2470: F, t4107: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t10041, t10044, t10062, t10065) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1299::<F>(t10039, t869, t689, t2777, t4092, t2439, t3923, t555, t4003, t5744, t2782, t4086, t543);
        let (t10066, t10069, t10070, t10073) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1300::<F>(t10065, t2782, t123, t212, t2434, t4089, t138, t2438, t785);
        let (t10074, t10080, t10085, t10098) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1301::<F>(t10073, t4089, t1398, t1419, t4086, t543, t2782, t4056, t555, t1432, t2470, t4107);
    (t10041, t10044, t10062, t10066, t10069, t10070, t10073, t10074, t10080, t10085, t10098)
}
