//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta223 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk976;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta223(t268: f64, t404: f64, t7021: f64, t159: f64, t3617: f64, t409: f64, t416: f64, t406: f64, t11335: f64, t281: f64, t414: f64, t3475: f64, t431: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t12295, t12296, t12305, t12327, t12331, t12349, t12351, t12352, t12367, t12382, t12397, t12428) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk976(t268, t404, t7021, t159, t3617, t409, t416, t406, t11335, t281, t414, t3475, t431);
    (t12295, t12296, t12305, t12327, t12331, t12349, t12351, t12352, t12367, t12382, t12397, t12428)
}
