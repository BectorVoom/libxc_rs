//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta586 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1915;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta586(t25365: f64, t28360: f64, t26485: f64, t99466: f64, t28377: f64, t689: f64, t25431: f64, t25411: f64, t102928: f64, t25387: f64, t28404: f64, t28384: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t102945, t102947, t102953, t102956, t102964, t102969, t102971, t102972) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1915(t25365, t28360, t26485, t99466, t28377, t689, t25431, t25411, t102928, t25387, t28404, t28384);
    (t102945, t102947, t102953, t102956, t102964, t102969, t102971, t102972)
}
