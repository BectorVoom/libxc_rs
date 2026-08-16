//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta492 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1791;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1792;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta492(t1385: f64, t2022: f64, t1426: f64, t545: f64, t7282: f64, t10073: f64, t2453: f64, t7283: f64, t136: f64, t2029: f64, t2457: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let t25931 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1791(t1385, t2022);
        let (t25937, t25938, t25939, t25941, t25944, t25945, t25946) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1792(t1426, t545, t2022, t7282, t10073, t2453, t7283, t136, t2029, t2457);
    (t25931, t25937, t25938, t25939, t25941, t25944, t25945, t25946)
}
