//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta530 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1881;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1882;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta530(t2014: f64, t28182: f64, t25190: f64, t7900: f64, t5542: f64, t7312: f64, t7315: f64, t7934: f64, t7235: f64, t7901: f64, t7937: f64, t2013: f64, t8995: f64, t2033: f64, t9593: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t28183, t28184, t28186, t28187, t28188, t28189, t28190, t28192, t28193, t28196) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1881(t2014, t28182, t25190, t7900, t5542, t7312, t7315, t7934, t7235, t7901, t7937, t2013, t8995);
        let t28197 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1882(t2033, t9593);
    (t28183, t28184, t28186, t28187, t28188, t28189, t28190, t28192, t28193, t28196, t28197)
}
