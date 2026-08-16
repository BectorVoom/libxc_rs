//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta425 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1522;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta425(t11354: f64, t6113: f64, t918: f64, t4598: f64, t4606: f64, t2880: f64, t6120: f64, t11358: f64, t4614: f64, t2897: f64, t18950: f64, t916: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t18980, t18982, t18985, t18988, t18990, t18993, t18995) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1522(t11354, t6113, t918, t4598, t4606, t2880, t6120, t11358, t4614, t2897, t18950, t916);
    (t18980, t18982, t18985, t18988, t18990, t18993, t18995)
}
